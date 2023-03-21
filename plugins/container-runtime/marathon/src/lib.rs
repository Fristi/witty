wit_bindgen::generate!("plugin-container-runtime" in "../../../wit");

use config::*;
use error::{HttpError, PluginError};
use container_runtime::{Container, ContainerRuntime};
use wit_bindgen::rt::vec::Vec;
use serde::Deserialize;
use serde::de::Error;
use http::{send, HttpRequest};
use serde_json_wasm::from_slice;

struct MarathonContainerRuntime;

#[derive(Deserialize)]
struct MarathonApps {
    apps: Vec<MarathonApp>
}

#[derive(Deserialize)]
struct MarathonApp {
    container: Option<String>
}

impl MarathonContainerRuntime {
    const FIELD_URL: &str = "url";
    const FIELD_KEY: &str = "key";
}

impl ContainerRuntime for MarathonContainerRuntime {
    fn containers() -> Result<Vec<Container>, PluginError> {
        let url = str(MarathonContainerRuntime::FIELD_URL).ok_or(PluginError::ConfigKeyNotFound(MarathonContainerRuntime::FIELD_URL.to_string()))?;
        let auth = secret(MarathonContainerRuntime::FIELD_KEY).ok_or(PluginError::ConfigKeyNotFound(MarathonContainerRuntime::FIELD_KEY.to_string()))?;

        let auth_header = format!("Basic {}", auth);
        let headers: [(&str, &str); 1] = [
            ("Authorization", auth_header.as_str())
        ];
        let marathon_url = format!("{}/v2/apps", url);

        let resp = send(HttpRequest { url: marathon_url.as_str(), headers: &headers })?;

        if resp.status == 200 {
            let res: MarathonApps = from_slice(resp.body.as_slice())?;

            let containers = res.apps
                .into_iter()
                .filter_map(|x| x.container)
                .map(|image| Container { image })
                .collect();

            return Ok(containers);
        } else {
            return Err(PluginError::Http(HttpError::InvalidResponse))
        }
    }

    fn config_discriptors() -> Vec<ConfigDescriptor> {
        vec![
            ConfigDescriptor {
                key: MarathonContainerRuntime::FIELD_URL.to_string(),
                kind: ConfigKind::Str,
            },
            ConfigDescriptor {
                key: MarathonContainerRuntime::FIELD_KEY.to_string(),
                kind: ConfigKind::Secret,
            }
        ]
    }
}


impl <T> From<T> for PluginError where T : Error {
    fn from(value: T) -> Self {
        PluginError::Json(value.to_string())
    }
}

export_plugin_container_runtime!(MarathonContainerRuntime);
