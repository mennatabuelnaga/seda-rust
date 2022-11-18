use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams},
    rpc_params,
    ws_client::WsClientBuilder,
};
use seda_adapters::MainChainAdapterTrait;
use serde_json::json;

use crate::{config::AppConfig, helpers::get_env_var, Result};

// TODO clean up jsonrpsee and replace with adapter.
// Maybe move this trait to adapters once that refactor happens.
#[async_trait::async_trait]
pub trait CliCommands: Send + Sync {
    type MainChainAdapter: MainChainAdapterTrait;

    fn new(config: AppConfig<Self::MainChainAdapter>) -> Self;

    async fn view_seda_server(&self, method: &str, params: ArrayParams) -> Result<String> {
        let seda_server_url = get_env_var("SEDA_SERVER_URL")?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;

        let response = client.request(method, params).await?;

        Ok(response)
    }

    async fn format_tx_and_request_seda_server(
        &self,
        method: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<<Self::MainChainAdapter as MainChainAdapterTrait>::FinalExecutionStatus>;

    async fn register_node(&self, socket_address: String) -> Result<()>;

    async fn remove_node(&self, node_id: u64) -> Result<()> {
        let method_name = "remove_node";

        let response = self
            .format_tx_and_request_seda_server(
                method_name,
                json!({ "node_id": node_id.to_string() }).to_string().into_bytes(),
                0_u128,
            )
            .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }

    async fn set_node_socket_address(&self, node_id: u64, new_socket_address: String) -> Result<()> {
        let method_name = "set_node_socket_address";

        let response = self
            .format_tx_and_request_seda_server(
                method_name,
                json!({ "node_id": node_id.to_string(), "new_socket_address": new_socket_address })
                    .to_string()
                    .into_bytes(),
                0_u128,
            )
            .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_node_socket_address(&self, node_id: u64) -> Result<()> {
        let near_server_url = get_env_var("NEAR_SERVER_URL")?;
        let contract_id = get_env_var("CONTRACT_ACCOUNT_ID")?;

        let response = self
            .view_seda_server(
                "get_node_socket_address",
                rpc_params![contract_id, node_id, near_server_url],
            )
            .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_nodes(&self, limit: u64, offset: u64) -> Result<()> {
        let near_server_url = get_env_var("NEAR_SERVER_URL")?;
        let contract_id = get_env_var("CONTRACT_ACCOUNT_ID")?;

        let response = self
            .view_seda_server("get_nodes", rpc_params![contract_id, limit, offset, near_server_url])
            .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_node_owner(&self, node_id: u64) -> Result<()> {
        let near_server_url = get_env_var("NEAR_SERVER_URL")?;
        let contract_id = get_env_var("CONTRACT_ACCOUNT_ID")?;

        let response = self
            .view_seda_server("get_node_owner", rpc_params![contract_id, node_id, near_server_url])
            .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }
}
