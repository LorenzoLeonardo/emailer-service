use async_curl::actor::CurlActor;
use curl_http_client::{
    collector::Collector, http_client::HttpClient, request::HttpRequest, response::HttpResponse,
};

use crate::error::EmailResult;

#[derive(Clone)]
pub struct Curl {
    pub actor_handle: CurlActor<Collector>,
}

impl Curl {
    pub fn new() -> Self {
        Self {
            actor_handle: CurlActor::new(),
        }
    }

    pub async fn send(&self, request: HttpRequest) -> EmailResult<HttpResponse> {
        log::debug!("Request Url: {}", request.url);
        log::debug!("Request Header: {:?}", request.headers);
        log::debug!("Request Method: {}", request.method);
        log::debug!("Request Body: {:?}", request.body);

        let response = HttpClient::new(Collector::RamAndHeaders(Vec::new(), Vec::new()))
            .request(request)?
            .nonblocking(self.actor_handle.clone())
            .perform()
            .await?;

        log::debug!("Response Header: {:?}", response.headers);
        log::debug!("Response Body: {:?}", response.body);
        log::debug!("Response Status: {}", response.status_code);
        Ok(response)
    }
}

impl Default for Curl {
    fn default() -> Self {
        Self::new()
    }
}
