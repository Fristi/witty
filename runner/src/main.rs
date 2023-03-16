wasmtime::component::bindgen!({ world: "plugin-enricher", path: "../wit", async: true});

use anyhow::*;
use std::time::Duration;

use commits::{CommitParam as Commit};
use enricher::Enrichment;
use context::Context;
use wasmtime::{
    component::{Component, InstancePre, Linker},
    Config, Engine, Store,
};
use wit_component::ComponentEncoder;

mod context;
mod reqwest_http;

#[derive(Clone)]
pub struct Worker {
    engine: Engine,
    pre_instance: InstancePre<Context>,
}

impl Worker {
    async fn from_url(url: &str) -> Result<Worker> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
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
        config::add_to_linker(&mut linker, Context::config)?;
        log::add_to_linker(&mut linker, Context::log)?;

        let pre_instance = linker.instantiate_pre(&component)?;

        Ok(Worker {
            engine,
            pre_instance,
        })
    }

    fn from_file(path: &str) -> Result<Worker> {
        let bytes = std::fs::read(path)?;
        Worker::from_bytes(bytes.as_slice())
    }

    pub async fn work(&self, commit: Commit<'_>) -> Result<Vec<Enrichment>> {
        let mut store: Store<Context> = Store::new(&self.engine, Context::new());
        let (gitlog, _) = PluginEnricher::instantiate_pre(&mut store, &self.pre_instance).await?;
        let res = gitlog.enricher.call_enrich(&mut store, commit).await?.unwrap();
        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let worker = Worker::from_file("/Users/markie/Projects/component-model-demo/target/wasm32-unknown-unknown/release/diffmrs_plugin_enrichment_jira.wasm")?;
    let mut handles = Vec::new();

    for _ in 1..2 {
        handles.push(tokio::spawn({
            let w = worker.clone();
            async move {
                w.work(Commit {
                    message: "SPA-1667: test",
                    timestamp: 0,
                })
                .await
            }
        }));
    }

    for h in handles {
        let res = h.await?.unwrap();
        dbg!("Got enrichment: {}", res);
    }

    Ok(())
}