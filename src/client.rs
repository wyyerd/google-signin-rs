use std::io::Read;

use error::Error;
use hyper::{self, client::RequestBuilder, net::HttpsConnector};
use serde;
use serde_json;
use token::IdInfo;

pub struct Client {
    client: hyper::Client,
    pub audiences: Vec<String>,
    pub hosted_domains: Vec<String>,
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://www.googleapis.com/oauth2/v3/{}", &path[1..])
    }

    #[cfg(feature = "with-rustls")]
    pub fn new() -> Client {
        use hyper_rustls::TlsClient;

        let tls = TlsClient::new();
        let connector = HttpsConnector::new(tls);
        let client = hyper::Client::with_connector(connector);
        Client { client, audiences: vec![], hosted_domains: vec![] }
    }

    #[cfg(feature = "with-openssl")]
    pub fn new() -> Client {
        use hyper_openssl::OpensslClient;

        let tls = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = hyper::Client::with_connector(connector);
        Client { client, audiences: vec![], hosted_domains: vec![] }
    }

    /// Verifies that the token is signed by Google's OAuth cerificate,
    /// and check that it has a valid issuer, audience, and hosted domain.
    ///
    /// Returns an error if the client has no configured audiences.
    pub fn verify(&self, id_token: &str) -> Result<IdInfo, Error> {
        // TODO: Switch to using local-verification with cache-control'd google certs
        //       instead of using `get_slow_unverified` (which is partially verified by Google.)
        //
        //       See https://github.com/wyyerd/google-signin-rs/issues/2 for more info.
        let id_info = self.get_slow_unverified(id_token)?;
        id_info.verify(self)?;
        Ok(id_info)
    }

    /// Checks the token using Google's slow OAuth-like authentication flow.
    ///
    /// This checks that the token is signed using Google's OAuth certificate,
    /// but does not check the issuer, audience, or other application-specific verifications.
    ///
    /// This is NOT the recommended way to use the library, but can be used in combination with
    /// [IdInfo.verify](https://docs.rs/google-signin/latest/google_signin/struct.IdInfo.html#impl)
    /// for applications with more complex error-handling requirements.
    pub fn get_slow_unverified(&self, id_token: &str) -> Result<IdInfo, Error> {
        self.get_any(&format!("/tokeninfo?id_token={}", id_token))
    }

    fn get_any<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Client::url(path);
        let request = self.client.get(&url);
        send(request)
    }
}

fn send<T: serde::de::DeserializeOwned>(request: RequestBuilder) -> Result<T, Error> {
    let mut response = request.send()?;
    let mut body = String::with_capacity(4096);
    response.read_to_string(&mut body)?;
    let status = response.status_raw().0;
    match status {
        200..=299 => {}
        _ => { return Err(Error::InvalidToken); }
    }

    serde_json::from_str(&body).map_err(|err| Error::from(err))
}
