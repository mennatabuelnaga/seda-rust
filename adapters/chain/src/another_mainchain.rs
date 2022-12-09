use std::sync::Arc;

use seda_config::MainChainConfig;

use super::errors::Result;
use crate::{MainChainAdapterTrait, TransactionParams};

#[derive(Debug)]
pub struct AnotherMainChain;

#[async_trait::async_trait]
impl MainChainAdapterTrait for AnotherMainChain {
    type Client = ();
    type Config = MainChainConfig;
    type FinalExecutionStatus = String;
    type SignedTransaction = String;

    fn new_client(_config: &Self::Config) -> Result<Self::Client> {
        Ok(())
    }

    async fn construct_signed_tx(
        _signer_acc_str: &str,
        _signer_sk_str: &str,
        _contract_id: &str,
        _method_name: &str,
        _args: Vec<u8>,
        _gas: u64,
        _deposit: u128,
        _server_url: &str,
    ) -> Result<Self::SignedTransaction> {
        Ok("construct_signed_tx".to_string())
    }

    async fn sign_tx(_client: Arc<Self::Client>, _tx_params: TransactionParams) -> Result<Self::SignedTransaction> {
        Ok("sign_tx".to_string())
    }

    async fn send_tx(
        _client: Arc<Self::Client>,
        _signed_tx: Self::SignedTransaction,
    ) -> Result<Self::FinalExecutionStatus> {
        Ok("send_tx".to_string())
    }

    async fn view(
        _client: Arc<Self::Client>,
        _contract_id: &str,
        _method_name: &str,
        _args: Vec<u8>,
    ) -> Result<String> {
        Ok("view".to_string())
    }
}
