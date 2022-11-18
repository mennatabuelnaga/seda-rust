use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use near_primitives::views::FinalExecutionStatus;
use seda_adapters::{MainChainAdapterTrait, NearMainChain};
use serde_json::json;

use super::cli_commands::CliCommands;
use crate::{config::AppConfig, errors::Result};

#[derive(Debug, Default)]
pub struct NearCliBackend;

#[async_trait::async_trait]
impl CliCommands for NearCliBackend {
    type MainChainAdapter = NearMainChain;

    async fn format_tx_and_request_seda_server(
        config: &AppConfig<Self::MainChainAdapter>,
        method: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<FinalExecutionStatus> {
        let seda_server_url = config
            .seda_server_url
            .as_ref()
            .ok_or("seda_server_url from cli, env var or config file.")?;
        let signer_acc_str = config
            .signer_account_id
            .as_ref()
            .ok_or("signer_account_id from cli, env var or config file.")?;
        let signer_sk_str = config
            .secret_key
            .as_ref()
            .ok_or("secret_key from cli, env var or config file.")?;
        let contract_id = config
            .contract_account_id
            .as_ref()
            .ok_or("contract_account_id from cli, env var or config file.")?;
        let near_server_url = config
            .main_chain_config
            .as_ref()
            .ok_or("Config [main_chain_config] section.")?
            .near_server_url
            .as_ref()
            .ok_or("near_server_url from config [main_chain_config] section.")?;
        let gas = config.gas.ok_or("gas from config.")?;

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
        let response = client.request(method, rpc_params![signed_tx, near_server_url]).await?;

        Ok(response)
    }

    async fn register_node(config: &AppConfig<Self::MainChainAdapter>, socket_address: String) -> Result<()> {
        let method_name = "register_node";
        let deposit = config
            .deposit_for_register_node
            .as_ref()
            .ok_or("deposit_for_register_node from config file.")?
            .parse()
            .expect("deposit_for_register_node from config file was not a valid number.");

        let response = Self::format_tx_and_request_seda_server(
            config,
            method_name,
            json!({ "socket_address": socket_address }).to_string().into_bytes(),
            deposit,
        )
        .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_node_socket_address(config: &AppConfig<Self::MainChainAdapter>, node_id: u64) -> Result<()> {
        let near_server_url = config
            .main_chain_config
            .as_ref()
            .ok_or("Config [main_chain_config] section.")?
            .near_server_url
            .as_ref()
            .ok_or("near_server_url from config [main_chain_config] section.")?;
        let contract_id = config
            .contract_account_id
            .as_ref()
            .ok_or("contract_account_id from cli, env var or config file.")?;

        let response = Self::view_seda_server(
            config,
            "get_node_socket_address",
            rpc_params![contract_id, node_id, near_server_url],
        )
        .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_nodes(config: &AppConfig<Self::MainChainAdapter>, limit: u64, offset: u64) -> Result<()> {
        let near_server_url = config
            .main_chain_config
            .as_ref()
            .ok_or("Config [main_chain_config] section.")?
            .near_server_url
            .as_ref()
            .ok_or("near_server_url from config [main_chain_config] section.")?;
        let contract_id = config
            .contract_account_id
            .as_ref()
            .ok_or("contract_account_id from cli, env var or config file.")?;

        let response = Self::view_seda_server(
            config,
            "get_nodes",
            rpc_params![contract_id, limit, offset, near_server_url],
        )
        .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }

    async fn get_node_owner(config: &AppConfig<Self::MainChainAdapter>, node_id: u64) -> Result<()> {
        let near_server_url = config
            .main_chain_config
            .as_ref()
            .ok_or("Config [main_chain_config] section.")?
            .near_server_url
            .as_ref()
            .ok_or("near_server_url from config [main_chain_config] section.")?;
        let contract_id = config
            .contract_account_id
            .as_ref()
            .ok_or("contract_account_id from cli, env var or config file.")?;

        let response = Self::view_seda_server(
            config,
            "get_node_owner",
            rpc_params![contract_id, node_id, near_server_url],
        )
        .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }
}
