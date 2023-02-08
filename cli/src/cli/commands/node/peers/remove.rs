use clap::Args;
use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use seda_config::AppConfig;

use crate::Result;

#[derive(Debug, Args)]
pub struct RemovePeer {
    /// A libp2p peer id (ex.
    /// 12D3KooWRg13CAzihqGpVfifoeK4nmZ15D3vpZSPfmaDT53CBr9R)
    pub peer_id: String,
}

impl RemovePeer {
    pub async fn handle(self, config: AppConfig) -> Result<()> {
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &config.seda_server_url))
            .await?;

        client.request("remove_peer", rpc_params!(&self.peer_id)).await?;
        println!("Peer {} has been removed", &self.peer_id);

        Ok(())
    }
}
