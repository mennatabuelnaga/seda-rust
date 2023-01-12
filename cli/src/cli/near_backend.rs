use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use seda_chain_adapters::{MainChainAdapterTrait, NearMainChain, NodeDetails, NodeIds};
use seda_config::NodeConfig;
use serde_json::json;
use tracing::debug;

use super::cli_commands::CliCommands;
use crate::errors::Result;

#[derive(Debug, Default)]
pub struct NearCliBackend;

#[async_trait::async_trait]
impl CliCommands for NearCliBackend {
    async fn format_tx_and_request_seda_server(
        seda_server_url: &str,
        near_chain_rpc_url: &str,
        node_config: &NodeConfig,
        method: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<Vec<u8>> {
        // TODO we should break the node config into other configs so you don't need the
        // entire node config for running the cli commands?
        // Might be fixed in PR where we improve CLI commands.
        let signer_acc_str = &node_config.signer_account_id;
        let signer_sk_str = &node_config.secret_key;
        let contract_id = &node_config.contract_account_id;
        let gas = node_config.gas;

        let signed_tx = NearMainChain::construct_signed_tx(
            signer_acc_str,
            signer_sk_str,
            contract_id,
            method,
            args,
            gas,
            deposit,
            near_chain_rpc_url,
        )
        .await?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;
        let response = client.request(method, rpc_params![signed_tx]).await?;
        Ok(response)
    }

    async fn register_node(
        seda_server_url: &str,
        near_chain_rpc_url: &str,
        node_config: &NodeConfig,
        socket_address: &str,
    ) -> Result<()> {
        let method_name = "register_node";

        let deposit = node_config.deposit;

        let response = Self::format_tx_and_request_seda_server(
            seda_server_url,
            near_chain_rpc_url,
            node_config,
            method_name,
            json!({ "socket_address": socket_address }).to_string().into_bytes(),
            deposit,
        )
        .await?;

        debug!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_node_socket_address(seda_server_url: &str, node_config: &NodeConfig, node_id: u64) -> Result<()> {
        let contract_id = &node_config.contract_account_id;

        let response = Self::view_seda_server(
            seda_server_url,
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

    async fn get_nodes(seda_server_url: &str, node_config: &NodeConfig, limit: u64, offset: u64) -> Result<()> {
        let contract_id = &node_config.contract_account_id;

        let response = Self::view_seda_server(
            seda_server_url,
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

    async fn get_node_owner(seda_server_url: &str, node_config: &NodeConfig, node_id: u64) -> Result<()> {
        let contract_id = &node_config.contract_account_id;

        let response = Self::view_seda_server(
            seda_server_url,
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
