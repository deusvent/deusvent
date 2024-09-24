//! Serializers responsible for serializing (and deserializing) different types of messages between client and API.
//!
//! There are 3 types of messages:
//! 1) Server messages are created on a server and send to clients. They are transferred as bincode data encoded
//!    in Base94 strings
//! 2) Public client messages are send from clients to server. They don't have any player specific information and
//!    don't contain any authentication tokens. They are encoded as JSON like {"k":[MESSAGE_TAG], "v":[MESSAGE_PAYLOAD]}
//! 3) Player signed messages. Such messages are player specific and includes player identifier (public_key) and also
//!    a signature for the payload and as a proof that player identifier is correct one

use binary_encoding::encode_message_tag;

use crate::encryption::{self, PrivateKey, PublicKey, PUBLIC_KEY_SIZE, SIGNATURE_SIZE};

/// Errors that may happen during data serializations
#[derive(Debug, uniffi::Error, thiserror::Error)]
pub enum SerializationError {
    /// Bad data which cannot be deserialized
    #[error("Data error: {msg}")]
    BadData {
        /// Error message
        msg: String,
    },
}

impl From<bincode::error::DecodeError> for SerializationError {
    fn from(err: bincode::error::DecodeError) -> Self {
        Self::BadData {
            msg: err.to_string(),
        }
    }
}

impl From<bincode::error::EncodeError> for SerializationError {
    fn from(err: bincode::error::EncodeError) -> Self {
        Self::BadData {
            msg: err.to_string(),
        }
    }
}

impl From<binary_encoding::EncodingError> for SerializationError {
    fn from(err: binary_encoding::EncodingError) -> Self {
        let binary_encoding::EncodingError::BadData(err) = err;
        SerializationError::BadData {
            msg: err.to_string(),
        }
    }
}

/// Serializer for client messages
pub struct ClientMessage;
impl ClientMessage {
    const JSON_PREFIX_START: &'static str = r#"{"k":""#;
    const JSON_PREFIX_END: &'static str = r#"","v":""#;
    const JSON_SUFFIX: &'static str = r#""}"#;

    fn json_prefix(tag: u16) -> String {
        format!(
            "{}{}{}",
            ClientMessage::JSON_PREFIX_START,
            encode_message_tag(tag),
            ClientMessage::JSON_PREFIX_END
        )
    }

    fn encode(data: &[u8], tag: u16) -> String {
        let mut output = ClientMessage::json_prefix(tag);
        output.push_str(&binary_encoding::encode_base94(data));
        output.push_str(ClientMessage::JSON_SUFFIX);
        output
    }

    fn decode(data: &str, tag: u16) -> Result<Vec<u8>, SerializationError> {
        let json_prefix = ClientMessage::json_prefix(tag);
        if !data.starts_with(&json_prefix) || !data.ends_with(ClientMessage::JSON_SUFFIX) {
            return Err(SerializationError::BadData {
                msg: "No json_prefix and json_suffix found".to_string(),
            });
        }
        let base64_data = &data[json_prefix.len()..data.len() - ClientMessage::JSON_SUFFIX.len()];
        let decoded_data = binary_encoding::decode_base94(base64_data)?;
        Ok(decoded_data)
    }

    /// Serialize client message using bincode, base94 and returns JSON string where "k" field has an
    /// encoded tag and "v" has an encoded payload
    pub fn serialize(msg: &impl bincode::Encode, tag: u16) -> Result<String, SerializationError> {
        let data = bincode::encode_to_vec(msg, bincode::config::standard())?;
        Ok(ClientMessage::encode(&data, tag))
    }

    /// Deserialize JSON string back to the client message type
    pub fn deserialize<T>(data: &str, tag: u16) -> Result<T, SerializationError>
    where
        T: bincode::Decode,
    {
        let decoded_data = ClientMessage::decode(data, tag)?;
        let instance: T = bincode::decode_from_slice(&decoded_data, bincode::config::standard())?.0;
        Ok(instance)
    }
}

/// Serializer for signed client messages which includes signature and player public_key identifier
pub struct SignedClientMessage;
impl SignedClientMessage {
    /// Serialize client message using bincode, base94 and returns JSON string where "k" field has an
    /// encoded tag and "v" has an encoded payload. Payload also includes public_key so that API
    /// can identify the player and signature to proof the public_key validity
    pub fn serialize(
        msg: &impl bincode::Encode,
        tag: u16,
        public_key: &PublicKey,
        private_key: &PrivateKey,
    ) -> Result<String, SerializationError> {
        let payload = bincode::encode_to_vec(msg, bincode::config::standard())?;
        let mut data = Vec::with_capacity(payload.len() + PUBLIC_KEY_SIZE + SIGNATURE_SIZE);
        data.extend_from_slice(&payload);
        data.extend_from_slice(&public_key.serialize());
        let signature = encryption::sign(&data, private_key);
        data.extend_from_slice(&signature);
        Ok(ClientMessage::encode(&data, tag))
    }

    /// Deserialize JSON string back to the pair of client message type and a player identifier string. Returns error if
    /// payload cannot be verified and signature is wrong
    pub fn deserialize<T>(data: &str, tag: u16) -> Result<(T, String), SerializationError>
    where
        T: bincode::Decode,
    {
        let decoded_data = ClientMessage::decode(data, tag)?;
        if decoded_data.len() < PUBLIC_KEY_SIZE + SIGNATURE_SIZE {
            return Err(SerializationError::BadData {
                msg: "Too short message".to_string(),
            });
        }
        let signature = &&decoded_data[decoded_data.len() - SIGNATURE_SIZE..];
        let public_key_data = &decoded_data[decoded_data.len() - SIGNATURE_SIZE - PUBLIC_KEY_SIZE
            ..decoded_data.len() - SIGNATURE_SIZE];
        let public_key = PublicKey::deserialize(public_key_data.to_vec())?;
        let signed_payload = &decoded_data[..decoded_data.len() - SIGNATURE_SIZE];
        if !encryption::verify(signed_payload, &public_key, signature) {
            return Err(SerializationError::BadData {
                msg: "Cannot verify the data".to_string(),
            });
        }
        let msg_data = &decoded_data[..decoded_data.len() - SIGNATURE_SIZE - PUBLIC_KEY_SIZE];
        let instance: T = bincode::decode_from_slice(msg_data, bincode::config::standard())?.0;
        Ok((instance, public_key.as_string()))
    }
}

/// Serializer for messages coming from the server to the client
pub struct ServerMessage;
impl ServerMessage {
    /// Serialize server message using bincode and Base94. First 2 bytes are message tag which
    /// allows clients efficiently check what kind of message it receive and deserialize it appropriately
    pub fn serialize(msg: &impl bincode::Encode, tag: u16) -> Result<String, SerializationError> {
        let data = bincode::encode_to_vec(msg, bincode::config::standard())?;
        let serialized = binary_encoding::encode_base94(&data);
        Ok(format!("{}{}", encode_message_tag(tag), serialized))
    }

    /// Deserialize string to the server message, it will return an error if supplied message tag
    /// doesn't match first two bytes of a message
    pub fn deserialize<T>(data: &str, tag: u16) -> Result<T, SerializationError>
    where
        T: bincode::Decode,
    {
        let message_tag = encode_message_tag(tag);
        if !data.starts_with(&message_tag) {
            return Err(SerializationError::BadData {
                msg: "Bad message tag".to_string(),
            });
        }
        let input = &data[message_tag.len()..];
        let decoded = binary_encoding::decode_base94(input)?;
        let instance: T = bincode::decode_from_slice(&decoded, bincode::config::standard())?.0;
        Ok(instance)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use serde_json::Value;

    use crate::{
        datetime::ServerTimestamp,
        messages::common::ping::{Ping, ServerStatus, Status},
    };

    use super::*;

    #[test]
    fn client_messages_serialization() {
        let msg = Ping { unused: false };
        let data = ClientMessage::serialize(&msg, 1).unwrap();
        assert_eq!(data, r#"{"k":"-.","v":" "}"#);
        let got: Ping = ClientMessage::deserialize(&data, 1).unwrap();
        assert_eq!(msg, got);

        // Ensure it's valid JSON
        let _: Value = serde_json::from_slice(data.as_bytes()).unwrap();
    }

    #[test]
    fn client_signed_message_serialization() {
        let msg = Ping { unused: false };
        let keys = encryption::generate_new_keys();
        let data =
            SignedClientMessage::serialize(&msg, 1, &keys.public_key, &keys.private_key).unwrap();

        // Ensure it's valid JSON
        let _: Value = serde_json::from_slice(data.as_bytes()).unwrap();

        // We can't assert for actual data as keys are generated, but length is constant
        assert_eq!(data.len(), 136);
        let parsed = SignedClientMessage::deserialize::<Ping>(&data, 1).unwrap();
        assert_eq!(msg, parsed.0);
        assert_eq!(keys.public_key.as_string(), parsed.1);

        // Signature is stable for the same content
        let data_repeat =
            SignedClientMessage::serialize(&msg, 1, &keys.public_key, &keys.private_key).unwrap();
        assert_eq!(data, data_repeat);

        // Signature differs for different content
        let msg = Ping { unused: true };
        let data_different_msg =
            SignedClientMessage::serialize(&msg, 1, &keys.public_key, &keys.private_key).unwrap();
        assert_ne!(data, data_different_msg);
        let parsed = SignedClientMessage::deserialize::<Ping>(&data_different_msg, 1).unwrap();
        assert_eq!(msg, parsed.0);
    }

    #[test]
    fn server_message_serialization() {
        let msg = ServerStatus {
            timestamp: Arc::new(ServerTimestamp::from_milliseconds_pure(1)),
            status: Status::OK,
        };
        let data = ServerMessage::serialize(&msg, 1).unwrap();
        assert_eq!(data, "-.#f");
        let got: ServerStatus = ServerMessage::deserialize(&data, 1).unwrap();
        assert_eq!(msg, got);
    }
}
