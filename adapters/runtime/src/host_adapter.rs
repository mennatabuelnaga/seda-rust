use seda_chain_adapters::MainChainAdapterTrait;
use seda_runtime_sdk::Chain;

use crate::Result;

#[async_trait::async_trait]
pub trait HostAdapter: Send {
    type MainChainAdapter: MainChainAdapterTrait;

    async fn db_get(key: &str) -> Result<Option<String>>;
    async fn db_set(key: &str, value: &str) -> Result<()>;
    async fn http_fetch(url: &str) -> Result<String>;

    async fn chain_call(chain: Chain, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<Option<String>>;

    async fn chain_view(chain: Chain, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String>;
}
