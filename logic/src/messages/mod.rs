//! Message structs that are used for communication between the client and server.
//!
//! We are using AWS API Gateway WebSockets, which don't support sending binary data
//! (only base64-encoded strings) and also provide no compression. Incoming messages to
//! API Gateway are expected to be JSON documents with a certain field serving as a routing key,
//! which is then used to choose the correct processing AWS Lambda. Outgoing messages, however,
//! can be simple strings.
//!
//! For serialization, we are using 'bincode'  with base94 encoding, but the behavior differs for client
//! and server messages. Unfortunately, using base94 reduces space savings that we might apply, but
//! in the future, we may move away from API Gateway and use raw binary data.
//!
//! For client messages sent to the API, we need JSON with a routing key, so messages are in the form:
//! {"k":"[MESSAGE_TAG]","v":"[SERIALIZED_DATA]"}, where `SERIALIZED_DATA` is bincode data encoded in Base94
//! and [MESSAGE_TAG] is the u16 tag assigned to the message also encoded in Base94.
//!
//! For server messages, they are base94-encoded strings in a form:
//! - The first two string bytes represent the message tag
//! - The remaining bytes are bincode-serialized data
//!
//! Having prefix with message tag allows clients to efficiently determine which message it received via
//! web socket and use correct deserialize logic.

pub mod common;
// pub mod game;

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

#[cfg(test)]
mod tests {

    #[test]
    fn check_max_message_tag() {
        // Useful for development to quickly find next available message tag when you want to add a new one
        let server_tag = messages_macro::max_server_message_type!();
        let client_tag = messages_macro::max_client_message_type!();
        println!("Maximum server message tag={}", server_tag);
        println!("Maximum client message tag={}", client_tag);
    }
}
