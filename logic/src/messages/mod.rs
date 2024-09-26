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
//! and [MESSAGE_TAG] is the u16 tag assigned to the message also encoded in custom binary encoding.
//!
//! For server messages, they are encoded strings in a form:
//! - The first two string bytes represent the message tag
//! - The remaining bytes are Base94 bincode-serialized data
//!
//! Having prefix with message tag allows clients to efficiently determine which message it received via
//! web socket and use correct deserialize logic.
//!
//! Encoding should be used only for message serialization for client/backend communication and should not
//! be used in long term storages as bincode is not backward or forward compatible.

use std::sync::Arc;

use serializers::SerializationError;

use crate::encryption::{PrivateKey, PublicKey};

pub mod common;
pub mod game;
pub mod serializers;

/// Trait for all public client messages, public meaning no authentication context is needed
pub trait ClientPublicMessage {
    /// Returns message tag
    fn tag() -> u16;

    /// Serialize message and request_id to string
    fn serialize(&self, request_id: u8) -> Result<String, SerializationError>;

    /// Deserialize string to message itself and request_id
    fn deserialize(input: String) -> Result<(Self, u8), SerializationError>
    where
        Self: std::marker::Sized;
}

/// Trait for all client player messages, meaning it comes with authentication context
pub trait ClientPlayerMessage {
    /// Returns message tag
    fn tag() -> u16;

    /// Serialize message and request_id to string
    fn serialize(
        &self,
        request_id: u8,
        public_key: PublicKey,
        private_key: PrivateKey,
    ) -> Result<String, SerializationError>;

    /// Deserialize string to message itself and request_id
    fn deserialize(input: String) -> Result<(Self, Arc<PublicKey>, u8), SerializationError>
    where
        Self: std::marker::Sized;
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
