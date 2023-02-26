// generate bindings.
wasmtime::component::bindgen!({ world: "git", path: "../wit/git.wit" });
use std::fmt::{Display, Formatter};

use anyhow::*;
use wasmtime::{component::{Component, Linker}, Engine, Store, Config};
// Acutally, when you run `cargo b` in the last step, you will get a wasm module,
// not a wasm component, so we need to use this ComponentEncoder to transform the
// wasm module to component.
use wit_component::ComponentEncoder;

fn main() -> Result<()> {
    let mut config = Config::new();
    // Enable component here.
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, 0);
    let linker = Linker::new(&engine);
    let wasi_adapter = include_bytes!("../../wit/wasi_snapshot_preview1.wasm");

    // we first read the bytes of the wasm module.
    let module = std::fs::read("./target/wasm32-unknown-unknown/release/app.wasm")?;
    // then we transform module to compoennt.
    // remember to get wasi_snapshot_preview1.wasm first.
    let component = ComponentEncoder::default()
        .module(module.as_slice())?
        .validate(true)
        .adapter("wasi_snapshot_preview1", wasi_adapter)?
        .encode()?;
    std::fs::write("./target/component.wasm", &component)?;
    let component = Component::from_binary(&engine, &component)?;

    // after getting the component, we can instantiate a markdown instance.
    let (gitlog, _) = Gitlog::instantiate(&mut store, &component, &linker)?;
    let res = gitlog.exports.call_enrich(&mut store, exports::Commit{ message: "TECH: added stuff", timestamp: 0})?.unwrap();
    println!("Got enrichment: {}", res);
    Ok(())
}

impl Display for exports::Enrichment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            exports::Enrichment::Link(link) => write!(f, "Link({})", link),
            exports::Enrichment::None => write!(f, "None")
        }
    }
}