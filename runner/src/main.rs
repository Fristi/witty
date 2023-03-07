wasmtime::component::bindgen!({ world: "git", path: "../wit/git.wit", async: true});

use std::fmt::{Display, Formatter};
use anyhow::*;

use wasmtime::{component::{Component, Linker}, Config, Engine, Store};
use wit_component::ComponentEncoder;
use context::Context;

mod context;
mod reqwest;

#[tokio::main]
async fn main() {
    let mut config = Config::new();
    // Enable component here.
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config).unwrap();
    let mut store: Store<Context> = Store::new(&engine, Context::new());
    let mut linker: Linker<Context> = Linker::new(&engine);

    http::add_to_linker(&mut linker, Context::http).unwrap();

    let module = std::fs::read("./target/wasm32-unknown-unknown/release/app.wasm").unwrap();

    let component = ComponentEncoder::default()
        .module(module.as_slice()).unwrap()
        .validate(true)
        .encode().unwrap();

    let component = Component::from_binary(&engine, &component).unwrap();

    // after getting the component, we can instantiate a markdown instance.
    let (gitlog, _) = Gitlog::instantiate_async(&mut store, &component, &linker).await.unwrap();
    let res = gitlog.data.call_enrich(&mut store, data::Commit{ message: "IP: test", timestamp: 0}).await.unwrap().unwrap();
    println!("Got enrichment: {}", res);
}

impl Display for data::Enrichment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            data::Enrichment::Link(link) => write!(f, "Link({})", link),
            data::Enrichment::None => write!(f, "None")
        }
    }
}
