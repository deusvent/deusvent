//! Decay game object and corresponding query for it

use std::sync::Arc;

use messages_macro::client_player_message;
use messages_macro::server_message;

use crate::datetime::Duration;
use crate::datetime::ServerTimestamp;

#[server_message(2)]
pub struct Decay {
    /// Starting timestamp of a Decay
    pub started_at: Arc<ServerTimestamp>,
    /// How long Decay takes time
    pub length: Arc<Duration>,
}

#[client_player_message(2)]
pub struct DecayQuery {}

#[uniffi::export]
impl DecayQuery {
    /// Create new DecayQuery message
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}
