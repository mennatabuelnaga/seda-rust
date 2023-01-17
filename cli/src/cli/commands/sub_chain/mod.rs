use clap::Subcommand;
use seda_config::ChainConfigs;
use seda_runtime_sdk::Chain;

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
    pub async fn handle(self, chains_config: ChainConfigs) -> Result<()> {
        match self {
            Self::Call(call) => call.handle(chains_config).await,
            Self::View(view) => view.handle(chains_config).await,
        }
    }
}
