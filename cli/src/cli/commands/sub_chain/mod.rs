use clap::Subcommand;
use seda_config::{AppConfig, PartialChainConfigs};

use crate::Result;

mod call;
mod view;

#[derive(Debug, Subcommand)]
pub enum SubChain {
    // ./seda sub-chain view near mc.mennat0.testnet get_nodes "{\"offset\":\"0\",\"limit\":\"2\"}"
    View(view::View),
    // ./seda sub-chain call near mc.mennat0.testnet register_node
    // "{\"socket_address\":\"127.0.0.1:8080\"}" "870000000000000000000"
    Call(call::Call),
}

impl SubChain {
    #[tokio::main]
    pub async fn handle(self, config: AppConfig, chains_config: PartialChainConfigs) -> Result<()> {
        match self {
            Self::Call(call) => call.handle(config, chains_config).await,
            Self::View(view) => view.handle(config, chains_config).await,
        }
    }
}
