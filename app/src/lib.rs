wit_bindgen::generate!("git" in "../wit/git.wit");

use serde::Deserialize;
use data::*;
use http::*;

struct MyGitlog;

#[derive(Deserialize)]
struct IpData {
    ip: String
}

impl Data for MyGitlog {
    fn enrich(commit: Commit) -> Result<Enrichment, EnrichmentError> {
        if commit.message.contains("TECH") {
            return Ok(Enrichment::Link("http://vectos.net".to_string()))
        } else if commit.message.contains("IP") {
            let headers: [(&str, &str); 0] = [];
            let resp = send( HttpRequest { url: "https://api.ipify.org?format=json", headers: &headers })?;

            if resp.status == 200 {
                let ip: IpData = serde_json::from_slice(resp.body.as_slice())?;
                return Ok(Enrichment::Link(ip.ip));
            } else {
                return Ok(Enrichment::None)
            }
        } else {
            return Ok(Enrichment::None)
        }
    }
}

impl From<http::HttpError> for EnrichmentError {
    fn from(value: http::HttpError) -> Self {
        EnrichmentError::HttpError(value)
    }
}

impl From<serde_json::Error> for EnrichmentError {
    fn from(value: serde_json::Error) -> Self {
        EnrichmentError::JsonError(value.to_string())
    }
}

export_gitlog!(MyGitlog);