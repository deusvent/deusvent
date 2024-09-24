use messages_macro::client_player_message;

use crate::encryption::SafeString;

#[client_player_message(3)]
pub struct Identity {
    pub name: SafeString,
}
