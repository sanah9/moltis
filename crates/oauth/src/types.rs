use serde::{Deserialize, Serialize};

/// OAuth 2.0 provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Extra query parameters to include in the authorization URL.
    #[serde(default)]
    pub extra_auth_params: Vec<(String, String)>,
    /// If true, use the GitHub device-flow instead of PKCE authorization code flow.
    #[serde(default)]
    pub device_flow: bool,
}

/// Stored OAuth tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    /// Unix timestamp when the access token expires.
    pub expires_at: Option<u64>,
}

/// PKCE challenge pair.
#[derive(Debug, Clone)]
pub struct PkceChallenge {
    pub verifier: String,
    pub challenge: String,
}
