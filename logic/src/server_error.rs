//! Communication with the server occurs via WebSockets. If the client sends a message that the server cannot process,
//! the server will respond with a `ServerError` message containing information about what went wrong.
//! This serves as a general error message. If there's a need for a custom error structure with additional information,
//! new custom server error messages can be defined and processed accordingly on the client side

use messages_macro::server_message;

use crate::messages::serializers::SerializationError;

/// General server error
#[server_message(3)]
pub struct ServerError {
    /// Error code
    pub error_code: ErrorCode,

    /// Error description that can be shown to the player. It should include a suggestion on how the error can be resolved
    pub error_description: String,

    /// Additional information about the error context, used only for debugging and not meant to be shown to users
    pub error_context: Option<String>,

    /// Message request identifier for which the error was created
    pub request_id: u8,

    /// Message tag for which the error was created
    pub message_tag: u16,

    /// Indicates whether the error is temporary and if the corresponding message can be safely retried
    pub recoverable: bool,
}

/// Error code
#[derive(uniffi::Enum, bincode::Encode, bincode::Decode, PartialEq, Debug, Clone)]
pub enum ErrorCode {
    /// Error with authentication or validating the signature
    AuthenticationError,

    /// Error ocurred during serialization or deserialization
    SerializationError,

    /// Read data is invalid
    InvalidData,

    /// Temporary IO error
    IOError,

    /// Undefined server error
    ServerError,
}

impl ServerError {
    /// Create new ServerError from SerializationError
    pub fn from_serialization_error(
        err: SerializationError,
        message_tag: u16,
        request_id: u8,
    ) -> Self {
        Self {
            error_code: ErrorCode::SerializationError,
            error_description: "Data is invalid and cannot be processed".to_string(),
            error_context: Some(err.to_string()),
            request_id,
            message_tag,
            recoverable: false,
        }
    }
}
