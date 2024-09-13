//! Health handler

use logic::{
    datetime::ServerTimestamp,
    messages::common::ping::{ServerStatus, Status},
};

/// Returns a message when server is health and operational
pub fn healthy_status(timestamp: ServerTimestamp) -> ServerStatus {
    ServerStatus {
        timestamp,
        status: Status::OK,
    }
}
