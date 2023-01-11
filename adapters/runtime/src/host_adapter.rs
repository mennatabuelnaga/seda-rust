use seda_chain_adapters::Client;
use seda_config::ChainConfigs;
use seda_runtime_sdk::Chain;

use crate::Result;

#[async_trait::async_trait]
pub trait HostAdapter: Send + Sync + Unpin + 'static {
    async fn new(config: ChainConfigs) -> Result<Self>
    where
        Self: Sized;

    fn select_client_from_chain(&self, chain: Chain) -> Client;

    async fn db_get(&self, key: &str) -> Result<Option<String>>;
    async fn db_set(&self, key: &str, value: &str) -> Result<()>;
    async fn http_fetch(&self, url: &str) -> Result<String>;

    async fn chain_call(
        &self,
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<Vec<u8>>;

    async fn chain_view(&self, chain: Chain, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String>;
}
