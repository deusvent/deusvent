//! Serializers responsible for serializing (and deserializing) different types of messages between client and API.
//!
//! There are 3 types of messages:
//! 1) Server messages are created on a server and send to clients. They are transferred as bincode data encoded
//!    in Base94
//! 2) Public client messages which are sent from clients to server. They don't have any player specific information and
//!    don't contain any authentication tokens. They are encoded as JSON like {"k":[MESSAGE_TAG],"v":[MESSAGE_PAYLOAD]}
//! 3) Player signed messages. Such messages are player specific and includes player identifier (public_key) and also
//!    a signature for the payload and as a proof that player identifier is correct one

use std::sync::Arc;

use binary_encoding::{decode_request_id, encode_message_tag, encode_request_id, REQUEST_ID_LEN};

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

/// Request id for the client messages. Server messages includes that so we can match it to the correct client requests
type RequestId = u8;

/// Unique player identifier equal to it's generated public key
#[derive(Debug, PartialEq, uniffi::Object)]
pub struct PlayerId(pub String);

/// Serializer for client public messages
pub struct ClientPublicMessage;
impl ClientPublicMessage {
    const JSON_PREFIX_START: &'static str = r#"{"k":""#;
    const JSON_PREFIX_END: &'static str = r#"","v":""#;
    const JSON_SUFFIX: &'static str = r#""}"#;

    fn json_prefix(tag: u16) -> String {
        format!(
            "{}{}{}",
            ClientPublicMessage::JSON_PREFIX_START,
            encode_message_tag(tag),
            ClientPublicMessage::JSON_PREFIX_END
        )
    }

    fn encode_to_string(data: &[u8], tag: u16) -> String {
        let mut output = ClientPublicMessage::json_prefix(tag);
        output.push_str(&binary_encoding::encode_base94(data));
        output.push_str(ClientPublicMessage::JSON_SUFFIX);
        output
    }

    fn decode_from_string(data: &str, tag: u16) -> Result<Vec<u8>, SerializationError> {
        let json_prefix = ClientPublicMessage::json_prefix(tag);
        if !data.starts_with(&json_prefix) || !data.ends_with(ClientPublicMessage::JSON_SUFFIX) {
            return Err(SerializationError::BadData {
                msg: "No json_prefix and json_suffix found".to_string(),
            });
        }
        let base64_data =
            &data[json_prefix.len()..data.len() - ClientPublicMessage::JSON_SUFFIX.len()];
        let decoded_data = binary_encoding::decode_base94(base64_data)?;
        Ok(decoded_data)
    }

    /// Serialize client message using bincode, base94 and returns JSON string where "k" field has an
    /// encoded tag and "v" has an encoded payload
    pub fn serialize(
        msg: &impl bincode::Encode,
        tag: u16,
        request_id: RequestId,
    ) -> Result<String, SerializationError> {
        let data = encode_to_binary(msg, request_id)?;
        Ok(ClientPublicMessage::encode_to_string(&data, tag))
    }

    /// Deserialize JSON string back to the client message type
    pub fn deserialize<T>(data: &str, tag: u16) -> Result<(T, RequestId), SerializationError>
    where
        T: bincode::Decode,
    {
        let data = ClientPublicMessage::decode_from_string(data, tag)?;
        decode_from_binary(&data)
    }
}

/// Serializer for signed client messages which includes signature and player public_key identifier
pub struct ClientPlayerMessage;
impl ClientPlayerMessage {
    /// Serialize client message using bincode, base94 and returns JSON string where "k" field has an
    /// encoded tag and "v" has an encoded payload. Payload also includes public_key so that API
    /// can identify the player and signature to proof the public_key validity
    pub fn serialize(
        msg: &impl bincode::Encode,
        tag: u16,
        request_id: RequestId,
        public_key: &PublicKey,
        private_key: &PrivateKey,
    ) -> Result<String, SerializationError> {
        let encoded_message = encode_to_binary(msg, request_id)?;
        let mut data = Vec::with_capacity(encoded_message.len() + PUBLIC_KEY_SIZE + SIGNATURE_SIZE);
        data.extend_from_slice(&encoded_message);
        data.extend_from_slice(&public_key.serialize());
        let signature = encryption::sign(&data, private_key);
        data.extend_from_slice(&signature);
        Ok(ClientPublicMessage::encode_to_string(&data, tag))
    }

    /// Deserialize JSON string back to the pair of client message type and a player identifier string. Returns error if
    /// payload cannot be verified and signature is wrong
    pub fn deserialize<T>(
        data: &str,
        tag: u16,
    ) -> Result<(T, Arc<PublicKey>, RequestId), SerializationError>
    where
        T: bincode::Decode,
    {
        let decoded_data = ClientPublicMessage::decode_from_string(data, tag)?;
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
        let (instance, request_id) = decode_from_binary(msg_data)?;
        Ok((instance, public_key, request_id))
    }
}

/// Serializer for messages coming from the server to the client
pub struct ServerMessage;
impl ServerMessage {
    /// Serialize server message using bincode and Base94. First 2 bytes are message tag, then next 2 bytes are
    /// request id. Having those prefixes allows clients efficiently check what kind of message it receive and
    /// process it appropriately
    pub fn serialize(
        msg: &impl bincode::Encode,
        tag: u16,
        request_id: RequestId,
    ) -> Result<String, SerializationError> {
        let data = bincode::encode_to_vec(msg, bincode::config::standard())?;
        let serialized = binary_encoding::encode_base94(&data);
        Ok(format!(
            "{}{}{}",
            encode_message_tag(tag),
            encode_request_id(request_id),
            serialized
        ))
    }

    /// Deserialize string to the server message, it will return an error if supplied message tag
    /// doesn't match first two bytes of a message
    pub fn deserialize<T>(data: &str, tag: u16) -> Result<(T, RequestId), SerializationError>
    where
        T: bincode::Decode,
    {
        let message_tag = encode_message_tag(tag);
        let total_len = message_tag.len() + REQUEST_ID_LEN;
        if data.len() < total_len {
            return Err(SerializationError::BadData {
                msg: "Data too short".to_string(),
            });
        }
        if !data.starts_with(&message_tag) {
            return Err(SerializationError::BadData {
                msg: "Bad message tag".to_string(),
            });
        }
        let request_id = decode_request_id(
            data[message_tag.len()..message_tag.len() + REQUEST_ID_LEN].as_bytes(),
        )?;
        let input = &data[message_tag.len() + REQUEST_ID_LEN..];
        let decoded = binary_encoding::decode_base94(input)?;
        let instance: T = bincode::decode_from_slice(&decoded, bincode::config::standard())?.0;
        Ok((instance, request_id))
    }
}

fn encode_to_binary(
    msg: &impl bincode::Encode,
    request_id: RequestId,
) -> Result<Vec<u8>, SerializationError> {
    let config = bincode::config::standard();

    // Manually pre-allocate vector with +1 capacity so that request_id would fit without reallocation along with encoded data
    let mut size_writer = bincode::enc::write::SizeWriter::default();
    bincode::encode_into_writer(msg, &mut size_writer, config)?;
    let msg_size = size_writer.bytes_written;
    let mut data = vec![0; msg_size + 1];

    // Now encode the message and add request_id itself as a last byte
    bincode::encode_into_slice(msg, &mut data, config)?;
    data[msg_size] = request_id;
    Ok(data)
}

fn decode_from_binary<T>(data: &[u8]) -> Result<(T, RequestId), SerializationError>
where
    T: bincode::Decode,
{
    if data.is_empty() {
        return Err(SerializationError::BadData {
            msg: "Data too short".to_string(),
        });
    }
    let request_id = data[data.len() - 1];
    let payload = &data[..&data.len() - 1];
    let instance: T = bincode::decode_from_slice(payload, bincode::config::standard())?.0;
    Ok((instance, request_id))
}

/// Decode string back to the request_id or fallback to 0 value
#[uniffi::export]
pub fn parse_request_id(data: String) -> RequestId {
    binary_encoding::decode_request_id(data.as_bytes()).unwrap_or({
        // Maybe it would make more sense to simply crash in this case as unparsable request id
        // means message is terribly broken or receiving logic is wrong
        0
    })
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
        let msg = Ping {};
        let data = ClientPublicMessage::serialize(&msg, 1, 1).unwrap();
        assert_eq!(data.len(), 18);
        assert_eq!(data, r#"{"k":"-.","v":"!"}"#);

        // Ensure deserialization works
        let got: (Ping, RequestId) = ClientPublicMessage::deserialize(&data, 1).unwrap();
        assert_eq!(got.0, msg);
        assert_eq!(got.1, 1);

        // Ensure it's valid JSON
        let _: Value = serde_json::from_slice(data.as_bytes()).unwrap();
    }

    #[test]
    fn client_signed_message_serialization() {
        let msg = Ping {};
        let keys = encryption::generate_new_keys();
        let data = ClientPlayerMessage::serialize(&msg, 1, 1, &keys.public_key, &keys.private_key)
            .unwrap();

        // Ensure it's valid JSON
        let _: Value = serde_json::from_slice(data.as_bytes()).unwrap();

        // We can't assert for actual data as keys are generated, but length is constant
        assert_eq!(data.len(), 136);
        let parsed = ClientPlayerMessage::deserialize::<Ping>(&data, 1).unwrap();
        assert_eq!(parsed.0, msg);
        assert_eq!(parsed.1.as_string(), keys.public_key.as_string());
        assert_eq!(parsed.2, 1);

        // Signature is stable for the same content
        let data_repeat =
            ClientPlayerMessage::serialize(&msg, 1, 1, &keys.public_key, &keys.private_key)
                .unwrap();
        assert_eq!(data, data_repeat);
    }

    #[test]
    fn server_message_serialization() {
        let msg = ServerStatus {
            timestamp: Arc::new(ServerTimestamp::from_milliseconds_pure(1)),
            status: Status::OK,
        };
        let data = ServerMessage::serialize(&msg, 1, 1).unwrap();
        assert_eq!(data, "-.-.#f");
        assert_eq!(data.len(), 6); // 2(tag) + 2(request_id) + 1(timestamp) + 1(status)
        let got: (ServerStatus, RequestId) = ServerMessage::deserialize(&data, 1).unwrap();
        assert_eq!(got.0, msg);
        assert_eq!(got.1, 1)
    }
}
