use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config};

#[derive(Debug, Serialize, Deserialize)]
pub struct NearConfig {
    pub near_server_url: Option<String>,
}

impl Config for NearConfig {
    fn template() -> Self {
        Self {
            near_server_url: Some("fill me in".to_string()),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.near_server_url, "NEAR_SERVER_URL");
    }
}

impl Default for NearConfig {
    fn default() -> Self {
        let mut this = Self { near_server_url: None };
        this.overwrite_from_env();
        this
    }
}
