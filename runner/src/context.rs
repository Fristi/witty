use anyhow::anyhow;

use crate::config;
use crate::log;
use crate::{reqwest_http::ReqwestHttp};
use std::time::Duration;

pub struct Context {
    http: ReqwestHttp,
    config: InMemoryConfig,
    log: ConsoleLog
}

impl Context {
    pub fn new() -> Self {
        Context {
            http: ReqwestHttp {
                timeout: Duration::from_secs(10),
            },
            config: InMemoryConfig {},
            log: ConsoleLog {}
        }
    }

    pub fn http(&mut self) -> &mut ReqwestHttp {
        &mut self.http
    }

    pub fn config(&mut self) -> &mut InMemoryConfig {
        &mut self.config
    }
    pub fn log(&mut self) -> &mut ConsoleLog {
        &mut self.log
    }
}

pub struct ConsoleLog;

#[async_trait::async_trait]
impl log::Host for ConsoleLog {
    async fn log(&mut self, level: log::Level, message: String) -> anyhow::Result<()> {
        Ok(println!("{:?} {}", level, message))
    }
}


pub struct InMemoryConfig;

#[async_trait::async_trait]
impl config::Host for InMemoryConfig  {
   async fn secret(&mut self, key: String) -> anyhow::Result<Option<String>> { Ok(Some("".to_string())) }
   async fn str(&mut self, key: String) -> anyhow::Result<Option<String>> { Ok(Some("".to_string())) }
   async fn unsignedint(&mut self, key: String) -> anyhow::Result<Option<u32>> { todo!() }
   async fn str_array(&mut self, key: String) -> anyhow::Result<Option<Vec<String>>> { Ok(Some(vec!["".to_string()])) }
}