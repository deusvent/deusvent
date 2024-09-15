//! Message structs that are used for communication between the client and server.
//!
//! We are using AWS API Gateway WebSockets, which don't support sending binary data
//! (only base64-encoded strings) and also provide no compression. Incoming messages to
//! API Gateway are expected to be JSON documents with a certain field serving as a routing key,
//! which is then used to choose the correct processing AWS Lambda. Outgoing messages, however,
//! can be simple base64 strings.
//!
//! For serialization, we are using 'bincode'  with base64 encoding, but the behavior differs for client
//! and server messages. Unfortunately, using base64 reduces any space savings that we might apply, but
//! in the future, we may move away from API Gateway and use raw binary data.
//!
//! For client messages sent to the API, we need JSON with a routing key, so messages are in the form:
//! {"k":"[MESSAGE_TYPE]","v":"[SERIALIZED_DATA]"}, where `SERIALIZED_DATA` is bincode data.
//!
//! For server messages, they are base64-encoded strings with encoded binary data in the following form:
//! - The first two bytes represent the message type
//! - The remaining bytes are bincode-serialized data
//!
//! The difference in formats is due to the fact that for server messages, JSON overhead is unnecessary,
//! and the ratio of server messages is expected to be much higher than client messages. In the future,
//! we may also consider adding compression for larger messages to further optimize bandwidth usage.

pub mod common;
// pub mod game;

/// Errors that may happen during data serializations
#[derive(Debug)]
pub enum SerializationError {
    /// Bad data which cannot be deserialized
    BadData(String),
}

/// Client message
pub trait ClientMessage {
    /// Returns message type which is used for message routing and deserialization
    fn message_type() -> u32;

    // /// Serialize message to the JSON string in the form:
    // /// {"k":"[MESSAGE_TYPE]","v":"[SERIALIZED_DATA]"}
    // fn serialize(&self) -> Result<String, SerializationError>;

    // /// Deserialize JSON message back to the message
    // fn deserialize(data: &[u8]) -> Result<Self, SerializationError>
    // where
    //     Self: Sized;
}

/// Base trait for all messages that are send between client and server
/// For serialization we use JSON for now, but trait is written in a way
/// to support binary protocols in the future
pub trait Message {}

#[cfg(test)]
mod tests {
    use common::ping::Ping;

    use super::*;

    #[test]
    fn test_message_type() {
        assert_eq!(Ping::message_type(), 0);
    }
}
