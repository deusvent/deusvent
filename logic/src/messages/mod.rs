//! Messages structs that are used for communication between client and server

pub mod auth;
pub mod game;

/// Errors that may happen during data serializations
#[derive(Debug)]
pub enum SerializationError {
    /// Bad data which cannot be deserialized
    BadData(String),
}

impl From<serde_json::Error> for SerializationError {
    fn from(err: serde_json::Error) -> Self {
        Self::BadData(err.to_string())
    }
}

/// Base trait for all messages that are send between client and server
/// For serialization we use JSON for now, but trait is written in a way
/// to support binary protocols in the future
pub trait Message {
    /// Returns message type which is used for message routing
    fn action_type() -> &'static str;

    /// Serialize message to the array of bytes for sending
    fn serialize(&self) -> Result<Vec<u8>, SerializationError>;

    /// Deserialize data to the message
    fn deserialize(data: &[u8]) -> Result<Self, SerializationError>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages_macro::message;

    #[message("foo.bar")]
    struct TestMsg {
        foo: String,
        bar: usize,
    }

    #[test]
    fn message_generation_test() {
        assert_eq!(TestMsg::action_type(), "foo.bar");
        let msg = TestMsg {
            foo: "foo".to_string(),
            bar: 42,
        };
        let data = msg.serialize().unwrap();
        let json = String::from_utf8(data.clone()).unwrap();
        assert_eq!(json, r#"{"foo":"foo","bar":42}"#);
        let got: TestMsg = TestMsg::deserialize(&data).unwrap();
        assert_eq!(msg, got);
    }
}
