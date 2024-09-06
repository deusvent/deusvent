//! Shared code related to the authentication

/// JWT based authentication token which is required for all player API calls
pub struct AuthToken {
    token: String,
}

impl AuthToken {
    /// Serialize JWT token to string
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Creates new auth token from the given string
    pub fn from_string(token: String) -> Self {
        // TODO Validate the token
        Self { token }
    }
}
