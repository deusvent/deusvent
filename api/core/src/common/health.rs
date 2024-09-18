//! Health handler

use std::sync::Arc;

use logic::{
    datetime::ServerTimestamp,
    messages::common::ping::{ServerStatus, Status},
};

/// Returns a message when server is health and operational
pub fn healthy_status(timestamp: ServerTimestamp) -> ServerStatus {
    ServerStatus {
        timestamp: Arc::new(timestamp),
        status: Status::OK,
    }
}
