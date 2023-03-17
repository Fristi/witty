use std::time::Duration;

use crate::http;
use crate::error::{HttpError, PluginError};

pub struct ReqwestHttp {
    pub timeout: Duration,
}

#[async_trait::async_trait]
impl http::Host for ReqwestHttp {
    async fn send(
        &mut self,
        req: http::HttpRequest,
    ) -> anyhow::Result<Result<http::HttpResponse, PluginError>> {
        let client = reqwest::Client::builder().timeout(self.timeout).build()?;

        let mut builder = client.get(req.url);

        for (k, v) in req.headers {
            builder = builder.header(k, v);
        }

        let resp = builder.send().await?;
        let status = resp.status().as_u16();
        let body = resp.bytes().await?.to_vec();

        Ok(Ok(http::HttpResponse { status, body }))
    }
}

impl From<reqwest::Error> for PluginError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_connect() || value.is_status() {
            return PluginError::Http(HttpError::Network);
        } else if value.is_timeout() {
            return PluginError::Http(HttpError::Timeout);
        }
        return PluginError::Http(HttpError::InvalidRequest);
    }
}
