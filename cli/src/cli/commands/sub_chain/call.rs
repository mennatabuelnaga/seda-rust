use clap::Args;
use seda_config::ChainConfigs;
use seda_node::ChainCall;
use seda_runtime_sdk::Chain;

use crate::Result;

#[derive(Debug, Args)]
pub struct Call {
    chain:       Chain,
    contract_id: String,
    method_name: String,
    args:        String,
    deposit:     u128,
}

impl Call {
    pub async fn handle(self, chains_config: ChainConfigs) -> Result<()> {
        // TODO make this a new function
        let call = ChainCall {
            chain: self.chain,
            contract_id: self.contract_id,
            method_name: self.method_name,
            args: self.args.into_bytes(),
            client: todo!(),
            deposit: self.deposit,
            // TODO we only need signer account id and secret key
            node_config: todo!(),
            chains_config,
        };
        let bytes = call.call_bytes().await?;
        return Ok(());
    }
}
