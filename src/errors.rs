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
            Error::Config(err) => write!(f, "[Error::Config] {}", err),
            Error::DomainService(err) => write!(f, "[Error::DomainService] {}", err),
            Error::Hyper(err) => write!(f, "[Error::Hyper] {}", err),
            Error::Io(err) => write!(f, "[Error::Io] {}", err),
            Error::IpAddr(err) => write!(f, "[Error::IpAddr] {}", err),
            Error::IpService(err) => write!(f, "[Error::IpService] {}", err),
            Error::NativeTls(err) => write!(f, "[Error::NativeTls] {}", err),
            Error::Request(err) => write!(f, "[Error::Request] {}", err),
            Error::Results(err) => write!(f, "[Error::Results] {}", err),
            Error::SerdeJson(err) => write!(f, "[Error::SerdeJson] {}", err),
            Error::Uri(err) => write!(f, "[Error::Uri] {}", err),
        }
    }
}
