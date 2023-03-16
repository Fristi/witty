wit_bindgen::generate!("plugin-enricher" in "../../../wit");

use enricher::{PluginError, Commit, Enricher, Enrichment};

struct JiraEnrichment;

impl Enricher for JiraEnrichment {
    fn enrich(_: Commit) -> Result<Enrichment, PluginError> {
        Ok(Enrichment::None)
    }
}

export_plugin_enricher!(JiraEnrichment);