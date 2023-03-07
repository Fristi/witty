use std::time::Duration;
use crate::reqwest::ReqwestHttp;

pub struct Context {
    http: ReqwestHttp
}

impl Context {
    pub fn new() -> Self {
        Context { http: ReqwestHttp { timeout: Duration::from_secs(10) } }
    }

    pub fn http(&mut self) -> &mut ReqwestHttp {
        &mut self.http
    }
}
