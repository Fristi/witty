wit_bindgen::generate!("git" in "../wit/git.wit");

use serde::Deserialize;

struct MyGitlog;

#[derive(Deserialize)]
struct IpData {
    ip: String
}

impl exports::Exports for MyGitlog {
    fn enrich(commit: exports::Commit) -> anyhow::Result<exports::Enrichment,String> {
        if commit.message.contains("TECH") {
            return Ok(exports::Enrichment::Link("http://vectos.net".to_string()))
        } else if commit.message.contains("IP") {
            let headers: [(&str, &str); 0] = [];
            let resp = http::send( http::HttpRequest { url: "https://api.ipify.org?format=json", headers: &headers });

            if resp.status == 200 {
                let ip: IpData = serde_json::from_slice(resp.body.as_slice()).unwrap();
                return Ok(exports::Enrichment::Link(ip.ip));
            } else {
                return Ok(exports::Enrichment::None)
            }
        } else {
            return Ok(exports::Enrichment::None)
        }
    }
}

export_gitlog!(MyGitlog);