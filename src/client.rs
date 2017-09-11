use error::Error;
use hyper;
use hyper::client::RequestBuilder;
use hyper::header::{Authorization, Basic, ContentType, Headers};
use hyper::net::HttpsConnector;
use serde;
use serde_json as json;
use std::io::Read;

pub struct Client {
    client: hyper::Client,
    secret_key: String,
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://api.stripe.com/v1/{}", &path[1..])
    }

    #[cfg(feature = "with-rustls")]
    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        use hyper_rustls::TlsClient;

        let tls = TlsClient::new();
        let connector = HttpsConnector::new(tls);
        let client = hyper::Client::with_connector(connector);
        Client {
            client: client,
            secret_key: secret_key.into(),
        }
    }

    #[cfg(feature = "with-openssl")]
    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        use hyper_openssl::OpensslClient;

        let tls = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = hyper::Client::with_connector(connector);
        Client {
            client: client,
            secret_key: secret_key.into(),
        }
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
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
        _ => { return Err(Error::Status(status)); }
    }

    json::from_str(&body).map_err(|err| Error::from(err))
}
