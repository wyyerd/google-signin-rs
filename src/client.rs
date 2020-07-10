use bytes::buf::ext::BufExt;
use hyper::client::{Client as HyperClient, HttpConnector};

#[cfg(feature = "with-openssl")]
use hyper_openssl::HttpsConnector;
#[cfg(feature = "with-rustls")]
use hyper_rustls::HttpsConnector;

use crate::cache_control::CacheControl;
use crate::certs::CachedCerts;
use crate::certs::Cert;
use crate::Error;
use crate::token::IdInfo;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use std::time::{Duration, Instant};

pub struct Client {
    client: HyperClient<HttpsConnector<HttpConnector>>,
    pub audiences: Vec<String>,
    pub hosted_domains: Vec<String>,
}

impl Default for Client {
    fn default() -> Self {
        #[cfg(feature = "with-rustls")]
        let ssl = HttpsConnector::new();
        #[cfg(feature = "with-openssl")]
        let ssl = HttpsConnector::new().expect("unable to build HttpsConnector");

        let client = HyperClient::builder()
            .http1_max_buf_size(0x2000)
            .pool_max_idle_per_host(0)
            .build(ssl);

        Client {
            client,
            audiences: vec![],
            hosted_domains: vec![],
        }
    }
}

impl Client {
    /// Verifies that the token is signed by Google's OAuth cerificate,
    /// and check that it has a valid issuer, audience, and hosted domain.
    /// Returns an error if the client has no configured audiences.
    pub async fn verify(
        &self,
        id_token: &str,
        cached_certs: &CachedCerts,
    ) -> Result<IdInfo, Error> {
        let unverified_header = jsonwebtoken::decode_header(&id_token)?;

        match unverified_header.kid {
            Some(kid) => {
                let cert = cached_certs.keys.get(&kid).ok_or(Error::InvalidKey)?;
                self.verify_single(id_token, cert)
            }
            None => cached_certs
                .keys
                .values()
                .flat_map(|cert| self.verify_single(id_token, cert))
                .next()
                .ok_or(Error::InvalidToken),
        }
    }

    fn verify_single(&self, id_token: &str, cert: &Cert) -> Result<IdInfo, Error> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&self.audiences);
        let token_data = jsonwebtoken::decode::<IdInfo>(
            &id_token,
            &DecodingKey::from_rsa_components(&cert.n, &cert.e),
            &validation,
        )?;

        token_data.claims.verify(self)?;

        Ok(token_data.claims)
    }

    /// Checks the token using Google's slow OAuth-like authentication flow.
    ///
    /// This checks that the token is signed using Google's OAuth certificate,
    /// but does not check the issuer, audience, or other application-specific verifications.
    ///
    /// This is NOT the recommended way to use the library, but can be used in combination with
    /// [IdInfo.verify](https://docs.rs/google-signin/latest/google_signin/struct.IdInfo.html#impl)
    /// for applications with more complex error-handling requirements.
    pub async fn get_slow_unverified(
        &self,
        id_token: &str,
    ) -> Result<IdInfo<String, String>, Error> {
        self.get_any(
            &format!(
                "https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}",
                id_token
            ),
            &mut None,
        )
        .await
    }

    pub(crate) async fn get_any<T: DeserializeOwned>(
        &self,
        url: &str,
        cache: &mut Option<Instant>,
    ) -> Result<T, Error> {
        let url = url.parse().unwrap();
        let response = self.client.get(url).await.unwrap();

        if !response.status().is_success() {
            return Err(Error::InvalidToken);
        }

        if let Some(value) = response.headers().get("Cache-Control") {
            if let Ok(value) = value.to_str() {
                if let Some(cc) = CacheControl::from_value(value) {
                    if let Some(max_age) = cc.max_age {
                        let seconds = max_age.as_secs();
                        *cache = Some(Instant::now() + Duration::from_secs(seconds as u64));
                    }
                }
            }
        }

        let body = hyper::body::aggregate(response).await?;
        Ok(serde_json::from_reader(body.reader())?)
    }
}
