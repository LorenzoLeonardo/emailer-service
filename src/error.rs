use std::fmt::Display;

use curl_http_client::collector::Collector;
use http::header::InvalidHeaderValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EmailError {
    Curl(String),
    Http(String),
    MailSend(String),
    Serde(String),
    IpcClient(String),
}

impl std::error::Error for EmailError {}

impl Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::Curl(err) => write!(f, "{}", err),
            EmailError::MailSend(err) => write!(f, "{}", err),
            EmailError::Http(err) => write!(f, "{}", err),
            EmailError::Serde(err) => write!(f, "{}", err),
            EmailError::IpcClient(err) => write!(f, "{}", err),
        }
    }
}

impl From<curl_http_client::error::Error<Collector>> for EmailError {
    fn from(value: curl_http_client::error::Error<Collector>) -> Self {
        EmailError::Curl(value.to_string())
    }
}

impl From<InvalidHeaderValue> for EmailError {
    fn from(e: InvalidHeaderValue) -> Self {
        EmailError::Http(e.to_string())
    }
}

impl From<serde_json::Error> for EmailError {
    fn from(e: serde_json::Error) -> Self {
        EmailError::Serde(e.to_string())
    }
}

impl From<json_elem::error::Error> for EmailError {
    fn from(e: json_elem::error::Error) -> Self {
        EmailError::Serde(e.to_string())
    }
}

impl From<ipc_client::client::error::Error> for EmailError {
    fn from(e: ipc_client::client::error::Error) -> Self {
        EmailError::IpcClient(e.to_string())
    }
}
pub type EmailResult<T> = Result<T, EmailError>;
