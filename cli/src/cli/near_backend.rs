use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use near_primitives::views::FinalExecutionStatus;
use seda_chain_adapters::{MainChainAdapterTrait, NearMainChain, NodeDetails, NodeIds};
use seda_config::CONFIG;
use serde_json::json;
use tracing::debug;

use super::cli_commands::CliCommands;
use crate::errors::Result;

#[derive(Debug, Default)]
pub struct NearCliBackend;

#[async_trait::async_trait]
impl CliCommands for NearCliBackend {
    type MainChainAdapter = NearMainChain;

    async fn format_tx_and_request_seda_server(
        method: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<FinalExecutionStatus> {
        let config = CONFIG.read().await;
        let node_config = config.node.as_ref().unwrap();
        let seda_server_url = config
            .seda_server_url
            .as_ref()
            .ok_or("seda_server_url from cli, env var or config file.")?;
        let signer_acc_str = node_config
            .signer_account_id
            .as_ref()
            .ok_or("signer_account_id from cli, env var or config file.")?;
        let signer_sk_str = node_config
            .secret_key
            .as_ref()
            .ok_or("secret_key from cli, env var or config file.")?;
        let contract_id = node_config
            .contract_account_id
            .as_ref()
            .ok_or("contract_account_id from cli, env var or config file.")?;
        let near_server_url = config
            .main_chain
            .as_ref()
            .ok_or("Config [main_chain] section.")?
            .near_server_url
            .as_ref()
            .ok_or("near_server_url from config [main_chain] section.")?;
        let gas = node_config
            .gas
            .as_ref()
            .ok_or("gas from config.")?
            .parse()
            .expect("deposit_for_register_node from config file was not a valid number.");

        let signed_tx = Self::MainChainAdapter::construct_signed_tx(
            signer_acc_str,
            signer_sk_str,
            contract_id,
            method,
            args,
            gas,
            deposit,
            near_server_url,
        )
        .await?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;
        let response = client.request(method, rpc_params![signed_tx]).await?;
        Ok(response)
    }

    async fn register_node(socket_address: String) -> Result<()> {
        let method_name = "register_node";

        let config = CONFIG.read().await;
        let node_config = config.node.as_ref().unwrap();
        let deposit = node_config
            .deposit_for_register_node
            .as_ref()
            .ok_or("deposit_for_register_node from config file.")?
            .parse()
            .expect("deposit_for_register_node from config file was not a valid number.");

        let response = Self::format_tx_and_request_seda_server(
            method_name,
            json!({ "socket_address": socket_address }).to_string().into_bytes(),
            deposit,
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_node_socket_address(node_id: u64) -> Result<()> {
        let config = CONFIG.read().await;
        let node_config = config.node.as_ref().unwrap();
        let contract_id = node_config
            .contract_account_id
            .clone()
            .ok_or("contract_account_id from cli, env var or config file.")?;

        let response =
            Self::view_seda_server("get_node_socket_address", rpc_params![NodeIds { contract_id, node_id }]).await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_nodes(limit: u64, offset: u64) -> Result<()> {
        let config = CONFIG.read().await;
        let node_config = config.node.as_ref().unwrap();
        let contract_id = node_config
            .contract_account_id
            .clone()
            .ok_or("contract_account_id from cli, env var or config file.")?;

        let response = Self::view_seda_server(
            "get_nodes",
            rpc_params![NodeDetails {
                contract_id,
                limit,
                offset
            }],
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_node_owner(node_id: u64) -> Result<()> {
        let config = CONFIG.read().await;
        let node_config = config.node.as_ref().unwrap();
        let contract_id = node_config
            .contract_account_id
            .clone()
            .ok_or("contract_account_id from cli, env var or config file.")?;

        let response = Self::view_seda_server("get_node_owner", rpc_params![NodeIds { contract_id, node_id }]).await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }
}
