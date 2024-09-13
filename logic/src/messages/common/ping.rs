//! Clients periodically sends Ping messages while server replies with ServerStatus

use messages_macro::message;
use serde::{Deserialize, Serialize};

use crate::datetime::ServerTimestamp;

/// Current server status
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    /// Everything is fine
    OK,
}

/// Server status message with common info like current time for time synchronization
#[message("common.serverStatus")]
pub struct ServerStatus {
    /// Current server timestamp, UTC
    pub timestamp: ServerTimestamp,
    /// Current server status
    pub status: Status,
}

/// Client ping message
#[message("common.ping")]
pub struct Ping {}
