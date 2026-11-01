use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    Config(String),
    DomainService(String),
    Hyper(String),
    Io(String),
    IpAddr(String),
    IpService(String),
    NativeTls(String),
    Request(String),
    Results(String),
    SerdeJson(String),
    Uri(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Config(err) => write!(f, "{}", err),
            Error::DomainService(err) => write!(f, "{}", err),
            Error::Hyper(err) => write!(f, "{}", err),
            Error::Io(err) => write!(f, "{}", err),
            Error::IpAddr(err) => write!(f, "{}", err),
            Error::IpService(err) => write!(f, "{}", err),
            Error::NativeTls(err) => write!(f, "{}", err),
            Error::Request(err) => write!(f, "{}", err),
            Error::Results(err) => write!(f, "{}", err),
            Error::SerdeJson(err) => write!(f, "{}", err),
            Error::Uri(err) => write!(f, "{}", err),
        }
    }
}
