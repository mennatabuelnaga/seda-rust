//! Defines a MainChainConfig type based on features when compiling.

mod near;
use std::sync::Arc;

use clap::Parser;
pub use near::*;

mod another_config;
pub use another_config::*;
use serde::{Deserialize, Serialize};

use crate::{Config, Result};

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
pub struct PartialChainConfigs {
    #[arg(skip)]
    pub another: PartialAnotherConfig,
    #[command(flatten)]
    pub near:    PartialNearConfig,
}

impl PartialChainConfigs {
    pub fn to_config(self, cli_options: PartialChainConfigs) -> Result<ChainConfigs> {
        Ok(Arc::new(ChainConfigsInner {
            another: self.another.to_config(),
            near:    self.near.to_config(cli_options.near)?,
        }))
    }
}

impl Config for PartialChainConfigs {
    fn template() -> Self {
        PartialChainConfigs {
            another: PartialAnotherConfig::template(),
            near:    PartialNearConfig::template(),
        }
    }

    fn overwrite_from_env(&mut self) {
        self.another.overwrite_from_env();
        self.near.overwrite_from_env();
    }
}

#[derive(Debug, Clone)]
pub struct ChainConfigsInner {
    pub another: AnotherConfig,
    pub near:    NearConfig,
}

pub type ChainConfigs = Arc<ChainConfigsInner>;
