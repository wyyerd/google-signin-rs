use client::Client;
use error::Error;

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    /// These six fields are included in all Google ID Tokens.
    pub iss: String,
    pub sub: String,
    pub azp: String,
    pub aud: String,
    pub iat: String,
    pub exp: String,

    /// This value indicates the user belongs to a Google Hosted Domain
    pub hd: Option<String>,

    /// These seven fields are only included when the user has granted the "profile" and
    /// "email" OAuth scopes to the application.
    pub email: Option<String>,
    pub email_verified: Option<String>,  // eg. "true" (but unusually as a string)
    pub name: Option<String>,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
}

impl TokenInfo {
    pub fn get(client: &Client, id_token: &str) -> Result<TokenInfo, Error> {
        client.get(&format!("/tokeninfo?id_token={}", id_token))
    }
}
