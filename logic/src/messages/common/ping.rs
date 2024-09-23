//! Clients periodically sends Ping messages while server replies with ServerStatus

use std::sync::Arc;

use messages_macro::{client_public_message, server_message};

use crate::datetime::ServerTimestamp;

/// Current server status
#[derive(Debug, Clone, PartialEq, bincode::Decode, bincode::Encode, uniffi::Enum)]
pub enum Status {
    /// Everything is fine
    OK,
}

/// Unix timestamp with milliseconds precision
#[derive(Debug, PartialEq, bincode::Decode, bincode::Encode)]
pub struct MyTimestamp(u64);

/// Server status message with common info like current time for time synchronization
#[server_message(1)]
pub struct ServerStatus {
    /// Current server timestamp, UTC
    pub timestamp: Arc<ServerTimestamp>,
    /// Current server status
    pub status: Status,
}

/// Client ping message
#[client_public_message(1)]
pub struct Ping {
    // HACK https://github.com/NordSecurity/uniffi-bindgen-cpp/issues/45
    pub unused: bool,
}
