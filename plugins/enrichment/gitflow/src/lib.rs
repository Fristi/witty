wit_bindgen::generate!("plugin-enricher" in "../../../wit");

use enricher::{Commit, ConfigDescriptor, Enricher, Enrichment, PluginError};
use wit_bindgen::rt::vec::Vec;

struct GitlogEnrichment;

impl Enricher for GitlogEnrichment {
    fn enrich(commit: Commit) -> Result<Enrichment, PluginError> {
        if commit.message.starts_with("feat:") {
            return Ok(Enrichment::Tag(String::from("Feature")));
        } else if commit.message.starts_with("hotfix:") {
            return Ok(Enrichment::Tag(String::from("Hotfix")));
        } else if commit.message.starts_with("chore:") {
            return Ok(Enrichment::Tag(String::from("Chore")));
        }

        return Ok(Enrichment::None);
    }

    fn config_discriptors() -> Vec<ConfigDescriptor> {
        vec![]
    }
}

export_plugin_enricher!(GitlogEnrichment);
