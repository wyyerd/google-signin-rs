extern crate hyper;
extern crate serde_json as json;

use std;

/// An error encountered when communicating with the Google API.
#[derive(Debug)]
pub enum Error {
    /// An error reported by the Google API.
    Status(u16),
    /// A networking error communicating with the Google server.
    Http(hyper::Error),
    /// An error reading the response body.
    Io(std::io::Error),
    /// An error converting between wire format and Rust types.
    Conversion(Box<std::error::Error + Send>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(std::error::Error::description(self))?;
        match *self {
            Error::Status(ref err) => write!(f, ": {}", err),
            Error::Http(ref err) => write!(f, ": {}", err),
            Error::Io(ref err) => write!(f, ": {}", err),
            Error::Conversion(ref err) => write!(f, ": {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Status(_) => "error reported by google api",
            Error::Http(_) => "error communicating with google servers",
            Error::Io(_) => "error reading response from google servers",
            Error::Conversion(_) => "error converting between wire format and Rust types",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Status(..) => None,
            Error::Http(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Conversion(ref err) => Some(&**err),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::Conversion(Box::new(err))
    }
}
