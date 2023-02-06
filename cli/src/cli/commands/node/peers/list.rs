use clap::Args;
use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use seda_config::AppConfig;

use crate::Result;

#[derive(Debug, Args)]
pub struct ListPeers;

impl ListPeers {
    pub async fn handle(self, config: AppConfig) -> Result<()> {
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &config.seda_server_url))
            .await?;

        client.request("list_peers", rpc_params!()).await?;

        Ok(())
    }
}
