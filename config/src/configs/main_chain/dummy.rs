use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config};

#[derive(Debug, Serialize, Deserialize)]
pub struct DummyConfig {
    pub chain_server_url: Option<String>,
}

impl Config for DummyConfig {
    fn template() -> Self {
        Self {
            chain_server_url: Some("fill me in".to_string()),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.chain_server_url, "CHAIN_SERVER_URL");
    }
}

impl Default for DummyConfig {
    fn default() -> Self {
        let mut this = Self { chain_server_url: None };
        this.overwrite_from_env();
        this
    }
}
