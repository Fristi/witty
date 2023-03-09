wasmtime::component::bindgen!({ world: "git", path: "../wit/git.wit", async: true});

use std::fmt::{Display, Formatter};
use std::time::Duration;
use anyhow::*;

use wasmtime::{component::{Component, Linker, InstancePre}, Config, Engine, Store};
use wit_component::ComponentEncoder;
use context::Context;
use crate::data::{Commit, Enrichment};

mod context;
mod reqwest_http;

#[derive(Clone)]
pub struct Worker {
    engine: Engine,
    pre_instance: InstancePre<Context>
}


impl Worker {

    async fn from_url(url: &str) -> Result<Worker> {
        let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;
        let resp = client.get(url).send().await?;
        let bytes = resp.bytes().await?;

        Worker::from_bytes(bytes.to_vec().as_slice())
    }

    fn from_bytes(module: &[u8]) -> Result<Worker> {
        let mut config = Config::new();
        // Enable component here.
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;
        let mut linker: Linker<Context> = Linker::new(&engine);

        let encoded_component = ComponentEncoder::default()
            .module(module)?
            .validate(true)
            .encode()?;

        let component = Component::from_binary(&engine, &encoded_component)?;

        http::add_to_linker(&mut linker, Context::http)?;

        let pre_instance = linker.instantiate_pre(&component)?;

        Ok(Worker { engine, pre_instance })
    }

    fn from_file(path: &str) -> Result<Worker> {
        let bytes = std::fs::read(path)?;
        Worker::from_bytes(bytes.as_slice())
    }

    pub async fn work(&self, commit: Commit<'_>) -> Result<Enrichment> {
        let mut store: Store<Context> = Store::new(&self.engine, Context::new());
        let (gitlog,_) = Gitlog::instantiate_pre(&mut store, &self.pre_instance).await?;
        let res = gitlog.data.call_enrich(&mut store, commit).await??;
        Ok(res)
    }
}


#[tokio::main]
async fn main() -> Result<()> {

    let worker = Worker::from_file("./target/wasm32-unknown-unknown/release/app.wasm")?;
    let mut handles = Vec::new();

    for _ in 1..10 {
        handles.push(tokio::spawn({
            let w = worker.clone();
            async move {
            w.work(Commit { message: "IP: test", timestamp: 0 }).await
        }}));
    }

    for h in handles {
        let res = h.await?.unwrap();
        println!("Got enrichment: {}", res);
    }


    Ok(())
}

impl Display for data::Enrichment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            data::Enrichment::Link(link) => write!(f, "Link({})", link),
            data::Enrichment::None => write!(f, "None")
        }
    }
}
