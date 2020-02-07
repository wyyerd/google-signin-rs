use hyper::{client::{Client as HyperClient, HttpConnector}};
#[cfg(feature = "with-rustls")]
use hyper_rustls::HttpsConnector;
#[cfg(feature = "with-openssl")]
use hyper_openssl::HttpsConnector;
use serde;
use serde_json;

use crate::error::Error;
use crate::token::IdInfo;
use bytes::buf::ext::BufExt;

pub struct Client {
    client: HyperClient<HttpsConnector<HttpConnector>>,
    pub audiences: Vec<String>,
    pub hosted_domains: Vec<String>,
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://www.googleapis.com/oauth2/v3/{}", &path[1..])
    }

    pub fn new() -> Client {
        #[cfg(feature = "with-rustls")]
        let ssl = HttpsConnector::new();
        #[cfg(feature = "with-openssl")]
        let ssl = HttpsConnector::new().expect("unable to build HttpsConnector");
        let client = HyperClient::builder().http1_max_buf_size(0x2000).keep_alive(false).build(ssl);
        Client { client, audiences: vec![], hosted_domains: vec![] }
    }

    /// Verifies that the token is signed by Google's OAuth cerificate,
    /// and check that it has a valid issuer, audience, and hosted domain.
    ///
    /// Returns an error if the client has no configured audiences.
    pub async fn verify(&self, id_token: &str) -> Result<IdInfo, Error> {
        // TODO: Switch to using local-verification with cache-control'd google certs
        //       instead of using `get_slow_unverified` (which is partially verified by Google.)
        //
        //       See https://github.com/wyyerd/google-signin-rs/issues/2 for more info.
        let id_info = self.get_slow_unverified(id_token).await?;
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
    pub async fn get_slow_unverified(&self, id_token: &str) -> Result<IdInfo, Error> {
        self.get_any(&format!("/tokeninfo?id_token={}", id_token)).await
    }

    async fn get_any<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Self::url(path).parse().unwrap();
        let response = self.client.get(url).await.unwrap();

        let status = response.status().as_u16();
        match status {
            200..=299 => {}
            _ => {
                return Err(Error::InvalidToken);
            }
        }

        let body = hyper::body::aggregate(response).await?;
        Ok(serde_json::from_reader(body.reader())?)
    }
}
