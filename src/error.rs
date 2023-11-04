use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EmailError {
    Curl(String),
    MailSend(String),
}

impl std::error::Error for EmailError {}

impl Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::Curl(err) => write!(f, "{}", err),
            EmailError::MailSend(err) => write!(f, "{}", err),
        }
    }
}

impl From<curl_http_client::error::Error> for EmailError {
    fn from(value: curl_http_client::error::Error) -> Self {
        EmailError::Curl(value.to_string())
    }
}

pub type EmailResult<T> = Result<T, EmailError>;
