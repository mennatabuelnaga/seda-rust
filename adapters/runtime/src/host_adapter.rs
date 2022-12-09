use seda_chain_adapters::MainChainAdapterTrait;

use crate::Result;

#[async_trait::async_trait]
pub trait HostAdapter: Send + Sync + Unpin + 'static {
    type MainChainAdapter: MainChainAdapterTrait;
    fn new() -> Result<Self>
    where
        Self: Sized;

    async fn db_get(&self, key: &str) -> Result<Option<String>>;
    async fn db_set(&self, key: &str, value: &str) -> Result<()>;
    async fn http_fetch(&self, url: &str) -> Result<String>;

    async fn chain_call(
        &self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<<Self::MainChainAdapter as MainChainAdapterTrait>::FinalExecutionStatus>;

    async fn chain_view(&self, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String>;
}
