use async_trait::async_trait;
use curl_http_client::{request::HttpRequest, response::HttpResponse};

use crate::error::EmailResult;

use super::{curl::Curl, Interface};

#[derive(Clone)]
pub struct Production {
    curl: Curl,
}

#[async_trait]
impl Interface for Production {
    async fn http_request(&self, request: HttpRequest) -> EmailResult<HttpResponse> {
        self.curl.send(request).await
    }
}

impl Production {
    pub fn new() -> Self {
        Self { curl: Curl::new() }
    }
}
