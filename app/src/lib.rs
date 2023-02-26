wit_bindgen::generate!("git" in "../wit/git.wit");

struct MyGitlog;

impl exports::Exports for MyGitlog {
    fn enrich(commit: exports::Commit) -> Result<exports::Enrichment,String> {
        if commit.message.contains("TECH") {
            return Ok(exports::Enrichment::Link("http://vectos.net".to_string()))
        } else if commit.message.contains("IP") {
            todo!()
        } else {
            return Ok(exports::Enrichment::None)
        }
    }
}

export_gitlog!(MyGitlog);