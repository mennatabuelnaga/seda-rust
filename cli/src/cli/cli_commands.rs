use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams, rpc_params},
    ws_client::WsClientBuilder,
};
use seda_config::NodeConfig;
use serde_json::json;
use tracing::debug;

use crate::Result;

// TODO clean up jsonrpsee and replace with adapter.
// Maybe move this trait to adapters once that refactor happens.
#[async_trait::async_trait]
pub trait CliCommands: Send + Sync {
    async fn view_seda_server(seda_server_url: &str, method: &str, params: ArrayParams) -> Result<String> {
        let client = WsClientBuilder::default()
            .build(&format!("ws://{seda_server_url}"))
            .await?;
        let response = client.request(method, params).await?;
        Ok(response)
    }

    async fn format_tx_and_request_seda_server(
        seda_server_url: &str,
        near_chain_rpc_url: &str,
        node_config: &NodeConfig,
        method: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<Vec<u8>>;

    async fn register_node(
        seda_server_url: &str,
        near_chain_rpc_url: &str,
        node_config: &NodeConfig,
        socket_address: &str,
    ) -> Result<()>;

    async fn remove_node(
        seda_server_url: &str,
        near_chain_rpc_url: &str,
        node_config: &NodeConfig,
        node_id: u64,
    ) -> Result<()> {
        let method_name = "remove_node";

        let response = Self::format_tx_and_request_seda_server(
            seda_server_url,
            near_chain_rpc_url,
            node_config,
            method_name,
            json!({ "node_id": node_id.to_string() }).to_string().into_bytes(),
            0_u128,
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn set_node_socket_address(
        seda_server_url: &str,
        near_chain_rpc_url: &str,
        node_config: &NodeConfig,
        node_id: u64,
        new_socket_address: &str,
    ) -> Result<()> {
        let method_name = "set_node_socket_address";

        let response = Self::format_tx_and_request_seda_server(
            seda_server_url,
            near_chain_rpc_url,
            node_config,
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

    async fn call_cli(seda_server_url: &str, args: &[String]) -> Result<()> {
        let client = WsClientBuilder::default()
            .build(&format!("ws://{seda_server_url}"))
            .await?;

        let response: Vec<String> = client.request("cli", rpc_params![args]).await?;

        response.iter().for_each(|s| print!("{s}"));

        Ok(())
    }

    async fn get_node_socket_address(seda_server_url: &str, node_config: &NodeConfig, node_id: u64) -> Result<()>;
    async fn get_nodes(seda_server_url: &str, node_config: &NodeConfig, limit: u64, offset: u64) -> Result<()>;
    async fn get_node_owner(seda_server_url: &str, node_config: &NodeConfig, node_id: u64) -> Result<()>;
}
