//! Clients periodically sends Ping messages while server replies with ServerStatus

use messages_macro::client_message;

use crate::datetime::ServerTimestamp;

/// Current server status
#[derive(Debug, PartialEq)]
pub enum Status {
    /// Everything is fine
    OK,
}

/// Server status message with common info like current time for time synchronization

pub struct ServerStatus {
    /// Current server timestamp, UTC
    pub timestamp: ServerTimestamp,
    /// Current server status
    pub status: Status,
}

/// Client ping message
#[client_message(0)]
pub struct Ping {}
