use clap::Args;
use seda_config::{AppConfig, PartialChainConfigs, PartialNodeConfig, PartialP2PConfig};

use crate::Result;

#[derive(Debug, Args)]
pub struct Run {
    #[command(flatten)]
    pub node_config:   PartialNodeConfig,
    #[command(flatten)]
    pub chains_config: PartialChainConfigs,
    #[command(flatten)]
    pub p2p_config:    PartialP2PConfig,
}

impl Run {
    pub fn handle(self, config: AppConfig) -> Result<()> {
        let node_config = config.node.to_config(self.node_config)?;
        let chains_config = config.chains.to_config(self.chains_config)?;
        let p2p_config = config.p2p.to_config(self.p2p_config)?;
        seda_node::run(&config.seda_server_url, node_config, p2p_config, chains_config);

        Ok(())
    }
}
