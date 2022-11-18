use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use near_primitives::views::FinalExecutionStatus;
use seda_adapters::{MainChainAdapterTrait, NearMainChain};
use serde_json::json;

use super::cli_commands::CliCommands;
use crate::{config::AppConfig, errors::Result};

#[derive(Debug, Default)]
pub struct NearCliBackend {
    config: AppConfig<NearMainChain>,
}

impl NearCliBackend {
    const DEPOSIT_FOR_REGISTER_NODE: u128 = 87 * 10_u128.pow(19);
    const GAS: u64 = 300_000_000_000_000; // 0.00087 NEAR
}

#[async_trait::async_trait]
impl CliCommands for NearCliBackend {
    type MainChainAdapter = NearMainChain;

    fn new(config: AppConfig<Self::MainChainAdapter>) -> Self {
        Self { config }
    }

    async fn format_tx_and_request_seda_server(
        &self,
        method: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<FinalExecutionStatus> {
        let seda_server_url = self.config.seda_server_url.as_ref().expect("TODO");
        let signer_acc_str = self.config.signer_account_id.as_ref().expect("TODO");
        let signer_sk_str = self.config.secret_key.as_ref().expect("TODO");
        let contract_id = self.config.contract_account_id.as_ref().expect("TODO");
        let near_server_url = self
            .config
            .main_chain_config
            .as_ref()
            .expect("TODO")
            .near_server_url
            .as_ref()
            .expect("TODO");

        let signed_tx = Self::MainChainAdapter::construct_signed_tx(
            signer_acc_str,
            signer_sk_str,
            contract_id,
            method,
            args,
            Self::GAS,
            deposit,
            near_server_url,
        )
        .await?;

        let client = WsClientBuilder::default().build(&seda_server_url).await?;
        let response = client.request(method, rpc_params![signed_tx, near_server_url]).await?;

        Ok(response)
    }

    async fn register_node(&self, socket_address: String) -> Result<()> {
        let method_name = "register_node";

        let response = self
            .format_tx_and_request_seda_server(
                method_name,
                json!({ "socket_address": socket_address }).to_string().into_bytes(),
                Self::DEPOSIT_FOR_REGISTER_NODE,
            )
            .await?;

        println!("response from server: {:?}", response);

        Ok(())
    }
}
