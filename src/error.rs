use hyper;
use serde_json;
use std::{self, fmt, io, sync::Arc};

/// A network or validation error
#[derive(Clone, Debug)]
pub enum Error {
    DecodeJson(Arc<serde_json::Error>),
    JSONWebToken(Arc<jsonwebtoken::errors::Error>),
    ConnectionError(Arc<dyn std::error::Error + Send + Sync + 'static>),
    InvalidKey,
    InvalidToken,
    InvalidIssuer,
    InvalidAudience,
    InvalidHostedDomain,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DecodeJson(ref err) => err.description(),
            Error::ConnectionError(ref err) => err.description(),
            Error::JSONWebToken(ref err) => err.description(),
            Error::InvalidKey => "invalid key",
            Error::InvalidToken => "invalid token",
            Error::InvalidIssuer => "invalid issuer",
            Error::InvalidAudience => "invalid audience",
            Error::InvalidHostedDomain => "invalid hosted domain",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::DecodeJson(ref err) => Some(&**err),
            Error::ConnectionError(ref err) => Some(&**err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DecodeJson(ref err) => err.fmt(f),
            Error::ConnectionError(ref err) => err.fmt(f),
            Error::JSONWebToken(ref err) => err.fmt(f),
            Error::InvalidKey => f.write_str("Token does not match any known key"),
            Error::InvalidToken => f.write_str("Token was not recognized by google"),
            Error::InvalidIssuer => f.write_str("Token was not issued by google"),
            Error::InvalidAudience => f.write_str("Token is for a different google application"),
            Error::InvalidHostedDomain => {
                f.write_str("User is not a member of the hosted domain(s)")
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::ConnectionError(Arc::new(err))
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::ConnectionError(Arc::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::DecodeJson(Arc::new(err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Error {
        Error::JSONWebToken(Arc::new(err))
    }
}
