use std::sync::Arc;

use messages_macro::client_player_message;
use messages_macro::server_message;

use crate::datetime::Duration;
use crate::datetime::ServerTimestamp;

#[server_message(2)]
pub struct Decay {
    started_at: Arc<ServerTimestamp>,
    length: Arc<Duration>,
}

#[client_player_message(2)]
pub struct DecayQuery {}
