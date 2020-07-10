use crate::{Client, Error};
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Deserialize)]
struct CertsObject {
    keys: Vec<Cert>,
}

#[derive(Deserialize)]
pub(crate) struct Cert {
    pub(crate) kid: String,
    pub(crate) e: String,
    pub(crate) n: String,
}

pub(crate) type Key = String;

#[derive(Default)]
pub struct CachedCerts {
    pub(crate) keys: HashMap<Key, Cert>,
    pub expiry: Option<Instant>,
}

impl CachedCerts {
    const CERTS_URL: &'static str = "https://www.googleapis.com/oauth2/v2/certs";

    /// Downloads the public Google certificates if it didn't do so already, or based on expiry of
    /// their Cache-Control. Returns `true` if the certificates were updated.
    pub async fn refresh_if_needed(&mut self) -> Result<bool, Error> {
        let refresh = self
            .expiry
            .map(|expiry| expiry <= Instant::now())
            .unwrap_or(true);

        if !refresh {
            return Ok(false);
        }

        let certs = Client::default()
            .get_any::<CertsObject>(Self::CERTS_URL, &mut self.expiry)
            .await?;
        self.keys.clear();

        for cert in certs.keys {
            self.keys.insert(cert.kid.clone(), cert);
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::CachedCerts;

    #[tokio::test]
    async fn google() {
        let mut certs = CachedCerts::default();

        let first = certs.refresh_if_needed().await.expect("failed");
        let second = certs.refresh_if_needed().await.expect("failed");
        assert_eq!(first, true);
        assert_eq!(second, false);
    }
}
