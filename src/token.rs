use crate::client::Client;
use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct IdInfo<EF=bool, TM=u64> {
    /// These six fields are included in all Google ID Tokens.
    pub iss: String,
    pub sub: String,
    pub azp: String,
    pub aud: String,
    pub iat: TM,
    pub exp: TM,

    /// This value indicates the user belongs to a Google Hosted Domain
    pub hd: Option<String>,

    /// These seven fields are only included when the user has granted the "profile" and
    /// "email" OAuth scopes to the application.
    pub email: Option<String>,
    pub email_verified: Option<EF>,  // eg. "true" (but unusually as a string)
    pub name: Option<String>,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
}

impl IdInfo {
    // Check the issuer, audiences, and (optionally) hosted domains of the IdInfo.
    //
    // Returns an error if the client has no configured audiences.
    pub fn verify(&self, client: &Client) -> Result<(), Error> {
        // Check the id was authorized by google
        match self.iss.as_str() {
            "accounts.google.com" | "https://accounts.google.com" => {}
            _ => { return Err(Error::InvalidIssuer); }
        }

        // Check the token belongs to the application(s)
        if client.audiences.len() > 0 && !client.audiences.contains(&self.aud) {
            return Err(Error::InvalidAudience);
        }

        // Check the token belongs to the hosted domain(s)
        if client.hosted_domains.len() > 0 {
            match self.hd {
                Some(ref domain) if client.hosted_domains.contains(domain) => {}
                _ => { return Err(Error::InvalidHostedDomain); }
            }
        }

        Ok(())
    }
}
