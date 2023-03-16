wit_bindgen::generate!("plugin-enricher" in "../../../wit");

use regex::Regex;
use config::*;
use error::{HttpError, PluginError};
use enricher::{Webref, Commit, Enricher, Enrichment};
use wit_bindgen::rt::vec::Vec;
use serde::Deserialize;
use serde::de::Error;
use http::{send, HttpRequest};
use serde_json_wasm::from_slice;

struct JiraEnrichment;

#[derive(Deserialize)]
struct JiraIssue {
    fields: JiraIssueFields
}

#[derive(Deserialize)]
struct JiraIssueFields {
    summary: String
}

impl JiraEnrichment {
    const FIELD_ORG: &str = "org";
    const FIELD_KEY: &str = "key";
    const FIELD_PROJECT_KEYS: &str = "project-keys";

    fn jira_api_issue_url(org: &str, key: &str) -> String {
        return format!("https://{}.atlassian.net/rest/api/2/issue/{}", org, key);
    }

    fn jira_issue_url(org: &str, key: &str) -> String {
        return format!("https://{}.atlassian.net/browse/{}", org, key);
    }

    fn get_jira_issue(org: &str, key: &str, auth: &str) -> Result<JiraIssue, PluginError> {
        let auth_header = format!("Basic {}", auth);
        let headers: [(&str, &str); 1] = [
            ("Authorization", auth_header.as_str())
        ];
        
        let resp = send(HttpRequest { url: &JiraEnrichment::jira_api_issue_url(&org, &key), headers: &headers })?;

        if resp.status == 200 {
            let issue: JiraIssue = from_slice(resp.body.as_slice())?;
            return Ok(issue);
        } else {
            return Err(PluginError::Http(HttpError::InvalidResponse))
        }
    }



}

impl Enricher for JiraEnrichment {


    fn enrich(commit: Commit) -> Result<Vec<Enrichment>, PluginError> {
        let org = str(JiraEnrichment::FIELD_ORG).ok_or(PluginError::ConfigKeyNotFound(JiraEnrichment::FIELD_ORG.to_string()))?;
        let keys = str_array(JiraEnrichment::FIELD_PROJECT_KEYS).ok_or(PluginError::ConfigKeyNotFound(JiraEnrichment::FIELD_PROJECT_KEYS.to_string()))?;
        let auth = secret(JiraEnrichment::FIELD_KEY).ok_or(PluginError::ConfigKeyNotFound(JiraEnrichment::FIELD_KEY.to_string()))?;
        let mut found_enrichment: Vec<Enrichment> = vec![];

        for key in keys {
            let regex = Regex::new(format!(r"({}-(\d+))", key).as_str())
                .map_err(|err| PluginError::Unexpected(err.to_string()))?;

            for cap in regex.captures_iter(commit.message.as_str()) {
                let jira_key = &cap[1];
                let issue = 
                    JiraEnrichment::get_jira_issue(org.as_str(), jira_key, &auth)?;

                found_enrichment.push(Enrichment::Link(Webref { label: issue.fields.summary, link: JiraEnrichment::jira_issue_url(&org, jira_key) }));
            }

        }

        return Ok(found_enrichment);
    }

    fn config_discriptors() -> Vec<ConfigDescriptor> {
        vec![
            ConfigDescriptor {
                key: JiraEnrichment::FIELD_ORG.to_string(),
                kind: ConfigKind::Str,
            },
            ConfigDescriptor {
                key: JiraEnrichment::FIELD_KEY.to_string(),
                kind: ConfigKind::Secret,
            },
            ConfigDescriptor {
                key: JiraEnrichment::FIELD_PROJECT_KEYS.to_string(),
                kind: ConfigKind::StrArray,
            },
        ]
    }
}


impl <T> From<T> for PluginError where T : Error {
    fn from(value: T) -> Self {
        PluginError::Json(value.to_string())
    }
}

export_plugin_enricher!(JiraEnrichment);
