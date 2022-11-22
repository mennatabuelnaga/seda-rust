use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams, rpc_params},
    ws_client::WsClientBuilder,
};
use seda_chain_adapters::MainChainAdapterTrait;
use serde_json::json;
use tracing::debug;

use crate::{config::AppConfig, Result};

// TODO clean up jsonrpsee and replace with adapter.
// Maybe move this trait to adapters once that refactor happens.
#[async_trait::async_trait]
pub trait CliCommands: Send + Sync {
    type MainChainAdapter: MainChainAdapterTrait;

    async fn view_seda_server(
        config: &AppConfig<Self::MainChainAdapter>,
        method: &str,
        params: ArrayParams,
    ) -> Result<String> {
        let seda_server_url = config
            .seda_server_url
            .as_ref()
            .ok_or("seda_server_url from cli, env var or config file.")?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;

        let response = client.request(method, params).await?;

        Ok(response)
    }

    async fn format_tx_and_request_seda_server(
        config: &AppConfig<Self::MainChainAdapter>,
        method: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<<Self::MainChainAdapter as MainChainAdapterTrait>::FinalExecutionStatus>;

    async fn register_node(config: &AppConfig<Self::MainChainAdapter>, socket_address: String) -> Result<()>;

    async fn remove_node(config: &AppConfig<Self::MainChainAdapter>, node_id: u64) -> Result<()> {
        let method_name = "remove_node";

        let response = Self::format_tx_and_request_seda_server(
            config,
            method_name,
            json!({ "node_id": node_id.to_string() }).to_string().into_bytes(),
            0_u128,
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn set_node_socket_address(
        config: &AppConfig<Self::MainChainAdapter>,
        node_id: u64,
        new_socket_address: String,
    ) -> Result<()> {
        let method_name = "set_node_socket_address";

        let response = Self::format_tx_and_request_seda_server(
            config,
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

    async fn call_cli(config: &AppConfig<Self::MainChainAdapter>, args: &[String]) -> Result<()> {
        let seda_server_url = config
            .seda_server_url
            .as_ref()
            .ok_or("seda_server_url from cli, env var or config file.")?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;

        let response: Vec<String> = client.request("cli", rpc_params![args]).await?;

        response.iter().for_each(|s| print!("{s}"));

        Ok(())
    }

    async fn call_cli(config: &AppConfig<Self::MainChainAdapter>, args: Vec<String>) -> Result<()> {
        let seda_server_url = config
            .seda_server_url
            .as_ref()
            .ok_or("seda_server_url from cli, env var or config file.")?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;

        let response: Vec<String> = client.request("cli", rpc_params![args]).await?;

        response.iter().for_each(|s| print!("{s}"));

        Ok(())
    }

    async fn get_node_socket_address(config: &AppConfig<Self::MainChainAdapter>, node_id: u64) -> Result<()>;
    async fn get_nodes(config: &AppConfig<Self::MainChainAdapter>, limit: u64, offset: u64) -> Result<()>;
    async fn get_node_owner(config: &AppConfig<Self::MainChainAdapter>, node_id: u64) -> Result<()>;
}
