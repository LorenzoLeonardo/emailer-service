pub mod curl;
#[cfg(test)]
pub mod mock;
pub mod production;

use async_trait::async_trait;

use curl_http_client::{request::HttpRequest, response::HttpResponse};

use crate::error::EmailResult;

#[async_trait]
pub trait Interface {
    async fn http_request(&self, request: HttpRequest) -> EmailResult<HttpResponse>;
}
