use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig};

use crate::Result;

#[derive(Debug, Args)]
pub struct Run {
    #[command(flatten)]
    pub node_config:   PartialNodeConfig,
    #[command(flatten)]
    pub chains_config: PartialChainConfigs,
}

impl Run {
    pub fn handle(self, config: AppConfig) -> Result<()> {
        let node_config = config.node.to_config(self.node_config)?;
        let chains_config = config.chains.to_config(self.chains_config)?;
        seda_node::run(&config.seda_server_url, node_config, chains_config);

        Ok(())
    }
}
