use serde::{Deserialize, Serialize};

use crate::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct DummyConfig;

impl Config for DummyConfig {
    fn template() -> Self {
        Self {}
    }

    fn overwrite_from_env(&mut self) {}
}

impl Default for DummyConfig {
    fn default() -> Self {
        let mut this = Self {};
        this.overwrite_from_env();
        this
    }
}
