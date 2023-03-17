wit_bindgen::generate!("plugin-enricher" in "../wit");

use serde::Deserialize;
use serde::de::Error;
use http::{send, HttpRequest};
use serde_json_wasm::from_slice;
use enricher::{PluginError, Commit, Enricher, Enrichment};


struct MyGitlog;

#[derive(Deserialize)]
struct IpData {
    ip: String
}

impl Enricher for MyGitlog {
    fn enrich(commit: Commit) -> Result<Enrichment, PluginError> {
        if commit.message.contains("TECH") {
            return Ok(Enrichment::Link("http://vectos.net".to_string()))
        } else if commit.message.contains("IP") {
            let headers: [(&str, &str); 0] = [];
            let resp = send( HttpRequest { url: "https://api.ipify.org?format=json", headers: &headers })?;

            if resp.status == 200 {
                let ip: IpData = from_slice(resp.body.as_slice())?;
                return Ok(Enrichment::Link(ip.ip));
            } else {
                return Ok(Enrichment::None)
            }
        } else {
            return Ok(Enrichment::None)
        }
    }
}

impl <T> From<T> for PluginError where T : Error {
    fn from(value: T) -> Self {
        PluginError::JsonError(value.to_string())
    }
}

export_plugin_enricher!(MyGitlog);