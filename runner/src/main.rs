// generate bindings.
wasmtime::component::bindgen!({ world: "git", path: "../wit/git.wit" });
use std::fmt::{Display, Formatter};
use std::io::Read;

use anyhow::*;
use wasmtime::{component::{Component, Linker}, Engine, Store, Config};
use wit_component::ComponentEncoder;

pub struct Context {
    http: ReqwestHttp
}

impl Context {
    pub fn new() -> Self {
        Context { http: ReqwestHttp { } }
    }

    pub fn http(&mut self) -> &mut ReqwestHttp {
        &mut self.http
    }
}

fn main() -> Result<()> {
    let mut config = Config::new();
    // Enable component here.
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store: Store<Context> = Store::new(&engine, Context::new());
    let mut linker: Linker<Context> = Linker::new(&engine);

    http::add_to_linker(&mut linker, Context::http)?;

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

    let component = Component::from_binary(&engine, &component)?;

    // after getting the component, we can instantiate a markdown instance.
    let (gitlog, _) = Gitlog::instantiate(&mut store, &component, &linker)?;
    let res = gitlog.exports.call_enrich(&mut store, exports::Commit{ message: "IP: test", timestamp: 0})?.unwrap();
    println!("Got enrichment: {}", res);
    Ok(())
}

pub struct ReqwestHttp;

impl http::Http for ReqwestHttp {


    fn send(&mut self, req:http::HttpRequest) -> anyhow::Result<http::HttpResponse> {

        let client = reqwest::blocking::Client::new();

        let mut builder = client.get(req.url);

        for (k, v) in req.headers {
            builder = builder.header(k, v);
        }

        let mut resp = builder.send()?;
        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf)?;

        Ok(http::HttpResponse{ status: resp.status().as_u16(), body: buf })
    }
}

impl Display for exports::Enrichment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            exports::Enrichment::Link(link) => write!(f, "Link({})", link),
            exports::Enrichment::None => write!(f, "None")
        }
    }
}