use async_trait::async_trait;
use curl_http_client::{request::HttpRequest, response::HttpResponse};
use http::{HeaderMap, StatusCode};

use crate::error::EmailResult;

use super::Interface;

#[derive(Clone)]
pub struct Mock {
    mock_response: HttpResponse,
}

#[async_trait]
impl Interface for Mock {
    async fn http_request(&self, _request: HttpRequest) -> EmailResult<HttpResponse> {
        Ok(self.mock_response.clone())
    }
}

impl Mock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            mock_response: HttpResponse {
                status_code: StatusCode::OK,
                headers: HeaderMap::new(),
                body: None,
            },
        }
    }
    #[allow(dead_code)]
    pub fn set_mock_response(mut self, response: HttpResponse) -> Self {
        self.mock_response = response;
        self
    }
}
