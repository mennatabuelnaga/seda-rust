use std::sync::Arc;

use seda_config::AnotherConfig;

use super::errors::Result;
use crate::MainChainAdapterTrait;

#[derive(Debug)]
pub struct AnotherMainChain;

#[async_trait::async_trait]
impl MainChainAdapterTrait for AnotherMainChain {
    type Client = ();
    type Config = AnotherConfig;

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
    ) -> Result<Vec<u8>> {
        unimplemented!()
    }

    async fn send_tx(_client: Arc<Self::Client>, _signed_tx: &[u8]) -> Result<Vec<u8>> {
        unimplemented!()
    }

    async fn view(
        _client: Arc<Self::Client>,
        _contract_id: &str,
        _method_name: &str,
        _args: Vec<u8>,
    ) -> Result<Vec<u8>> {
        unimplemented!()
    }
}
