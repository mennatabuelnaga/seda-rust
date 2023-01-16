use clap::Subcommand;
use seda_runtime_sdk::Chain;

use crate::Result;

#[derive(Debug, Subcommand)]
pub enum SubChain {
    // ./seda sub-chain view near mc.mennat0.testnet get_nodes "{\"offset\":\"0\",\"limit\":\"2\"}"
    View {
        chain:       Chain,
        contract_id: String,
        method_name: String,
        args:        String,
    },
    // ./seda sub-chain call near mc.mennat0.testnet register_node
    // "{\"socket_address\":\"127.0.0.1:8080\"}" "870000000000000000000"
    Call {
        chain:       Chain,
        contract_id: String,
        method_name: String,
        args:        String,
        deposit:     String,
    },
}

impl SubChain {
    pub fn handle(self) -> Result<()> {
        unimplemented!("")
    }
}
