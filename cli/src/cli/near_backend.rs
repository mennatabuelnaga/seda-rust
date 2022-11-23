use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use seda_chain_adapters::{MainChainAdapterTrait, NearMainChain, NodeDetails, NodeIds};
use seda_config::CONFIG;
use serde_json::json;
use tracing::debug;

use super::cli_commands::CliCommands;
use crate::errors::Result;

#[derive(Debug, Default)]
pub struct NearCliBackend;

// It's safe to call unwrap on the sub configs
// in the functions below. THis is because we
// already check they exist in the CLI.
#[async_trait::async_trait]
impl CliCommands for NearCliBackend {
    async fn format_tx_and_request_seda_server(method: &str, args: Vec<u8>, deposit: u128) -> Result<Vec<u8>> {
        let config = CONFIG.read().await;
        let node_config = &config.node;
        let seda_server_url = &config.seda_server_url;
        let signer_acc_str = &node_config.signer_account_id;
        let signer_sk_str = &node_config.secret_key;
        let contract_id = &node_config.contract_account_id;
        let chain_rpc_url = &config.near_chain.chain_rpc_url;
        let gas = node_config
            .gas
            .parse()
            .map_err(|e| format!("gas from config file was not a valid number: '{e}'."))?;

        let signed_tx = NearMainChain::construct_signed_tx(
            signer_acc_str,
            signer_sk_str,
            contract_id,
            method,
            args,
            gas,
            deposit,
            chain_rpc_url,
        )
        .await?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;
        let response = client.request(method, rpc_params![signed_tx]).await?;
        Ok(response)
    }

    async fn register_node(socket_address: String) -> Result<()> {
        let method_name = "register_node";

        let config = CONFIG.read().await;
        let node_config = &config.node;
        let deposit = node_config
            .deposit
            .parse()
            .map_err(|e| format!("deposit from config file was not a valid number: '{e}'."))?;

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
        let node_config = &config.node;
        let contract_id = &node_config.contract_account_id;

        let response = Self::view_seda_server(
            "get_node_socket_address",
            rpc_params![NodeIds {
                contract_id: contract_id.to_string(),
                node_id
            }],
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_nodes(limit: u64, offset: u64) -> Result<()> {
        let config = CONFIG.read().await;
        let node_config = &config.node;
        let contract_id = &node_config.contract_account_id;

        let response = Self::view_seda_server(
            "get_nodes",
            rpc_params![NodeDetails {
                contract_id: contract_id.to_string(),
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
        let node_config = &config.node;
        let contract_id = &node_config.contract_account_id;

        let response = Self::view_seda_server(
            "get_node_owner",
            rpc_params![NodeIds {
                contract_id: contract_id.to_string(),
                node_id
            }],
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }
}
