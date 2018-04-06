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

    pub fn verify(&self, id_token: &str) -> Result<IdInfo, Error> {
        // TODO: Use JWT to verify the token with cache-control'd google certs
        let id_info = self.get_unsafe(id_token)?;
        id_info.verify(self)?;
        Ok(id_info)
    }

    pub fn get_unsafe(&self, id_token: &str) -> Result<IdInfo, Error> {
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
        200...299 => {}
        _ => { return Err(Error::InvalidToken); }
    }

    serde_json::from_str(&body).map_err(|err| Error::from(err))
}
