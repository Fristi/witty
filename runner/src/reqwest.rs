use std::time::Duration;

use async_trait::async_trait;
use crate::http;

pub struct ReqwestHttp {
    pub timeout: Duration
}

#[async_trait]
impl http::Http for ReqwestHttp {
    async fn send(&mut self, req:http::HttpRequest) -> anyhow::Result<Result<http::HttpResponse, http::HttpError>> {

        let client = reqwest::Client::builder().timeout(self.timeout).build()?;

        let mut builder = client.get(req.url);

        for (k, v) in req.headers {
            builder = builder.header(k, v);
        }

        let resp = builder.send().await?;
        let status = resp.status().as_u16();
        let body = resp.bytes().await?.to_vec();


        Ok(Ok(http::HttpResponse{ status, body }))
    }
}

impl From<reqwest::Error> for http::HttpError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_connect() || value.is_status() {
            return http::HttpError::Network;
        } else if value.is_timeout() {
            return http::HttpError::Timeout;
        }

        return http::HttpError::InvalidRequest;
    }
}
