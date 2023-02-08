use clap::Args;
use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use seda_config::AppConfig;
use serde_json::Value;

use crate::Result;

#[derive(Debug, Args)]
pub struct ListPeers;

impl ListPeers {
    pub async fn handle(self, config: AppConfig) -> Result<()> {
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &config.seda_server_url))
            .await?;

        let response: Value = client.request("list_peers", rpc_params!()).await?;

        serde_json::to_writer_pretty(std::io::stdout(), &response)?;

        Ok(())
    }
}
