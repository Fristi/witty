wit_bindgen_guest_rust::generate!("../wits/git.wit");

struct Gitlog;

export_gitlog!(Gitlog);

impl gitlog::Gitlog for Gitlog {
    fn enrich(commit: gitlog::Commit) -> gitlog::Enrichment {
        if commit.message.contains("TECH") {
            gitlog::Enrichment::Link("http://vectos.net".to_string())
        } else {
            gitlog::Enrichment::None
        }
    }
}