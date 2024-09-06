//! Messages related to registration

use messages_macro::message;

/// Automatic client registration command. Issued on a first client run to
/// automatically create a user profile
#[message("command.auth.register")]
pub struct Register {
    /// Game client version in the form of "1.0.0"
    pub client_version: String,
}
