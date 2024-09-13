//! Health handler

use logic::{
    messages::common::ping::{ServerStatus, Status},
    time::ServerTimestamp,
};

/// Returns a message when server is health and operational
pub fn healthy_status(timestamp: ServerTimestamp) -> ServerStatus {
    ServerStatus {
        timestamp,
        status: Status::OK,
    }
}
