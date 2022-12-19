use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams, rpc_params},
    ws_client::WsClientBuilder,
};
use seda_config::CONFIG;
use serde_json::json;
use tracing::debug;

use crate::Result;

// TODO clean up jsonrpsee and replace with adapter.
// Maybe move this trait to adapters once that refactor happens.
#[async_trait::async_trait]
pub trait CliCommands: Send + Sync {
    async fn view_seda_server(method: &str, params: ArrayParams) -> Result<String> {
        let config = CONFIG.read().await;

        let seda_server_url = &config.seda_server_url;

        let client = WsClientBuilder::default().build(seda_server_url).await?;
        let response = client.request(method, params).await?;
        Ok(response)
    }

    async fn format_tx_and_request_seda_server(method: &str, args: Vec<u8>, deposit: u128) -> Result<Vec<u8>>;

    async fn register_node(socket_address: String) -> Result<()>;

    async fn remove_node(node_id: u64) -> Result<()> {
        let method_name = "remove_node";

        let response = Self::format_tx_and_request_seda_server(
            method_name,
            json!({ "node_id": node_id.to_string() }).to_string().into_bytes(),
            0_u128,
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn set_node_socket_address(node_id: u64, new_socket_address: String) -> Result<()> {
        let method_name = "set_node_socket_address";

        let response = Self::format_tx_and_request_seda_server(
            method_name,
            json!({ "node_id": node_id.to_string(), "new_socket_address": new_socket_address })
                .to_string()
                .into_bytes(),
            0_u128,
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn call_cli(args: &[String]) -> Result<()> {
        let config = CONFIG.read().await;
        let seda_server_url = &config.seda_server_url;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;

        let response: Vec<String> = client.request("cli", rpc_params![args]).await?;

        response.iter().for_each(|s| print!("{s}"));

        Ok(())
    }

    async fn get_node_socket_address(node_id: u64) -> Result<()>;
    async fn get_nodes(limit: u64, offset: u64) -> Result<()>;
    async fn get_node_owner(node_id: u64) -> Result<()>;
}
