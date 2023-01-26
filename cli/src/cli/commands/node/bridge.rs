use clap::Args;
use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use seda_config::{AppConfig, PartialChainConfigs};
use seda_runtime_sdk::Chain;

use crate::Result;

#[derive(Debug, Args)]
pub struct Bridge {
    #[arg(short, long)]
    pub chain:                 Chain,
    #[arg(long)]
    pub sub_chain_contract_id: String,
    #[arg(long)]
    pub sub_chain_method_name: String,
    #[arg(long)]
    pub bridge_deposit:        u128,
    #[arg(short, long)]
    pub args:                  String,
}

impl Bridge {
    pub async fn handle(self, config: AppConfig, _chains_config: PartialChainConfigs) -> Result<()> {
        // we don't need to validate configs here because they are using the one's from
        // when the Node was built. unless we should update it to use the one
        // when run from here? but that doesn't make sense to me.
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &config.seda_server_url))
            .await?;
        let args: Vec<String> = vec![
            "bridge".to_string(),
            self.chain.to_string(),
            self.sub_chain_contract_id,
            self.sub_chain_method_name,
            self.bridge_deposit.to_string(),
            self.args,
        ];

        let response: Vec<String> = client.request("cli", rpc_params!(args)).await?;
        // This is assuming we are always putting valid json in our wasm output...
        // This formatting is also a bit awkward...
        serde_json::to_writer_pretty(std::io::stdout(), &response)?;
        Ok(())
    }
}
