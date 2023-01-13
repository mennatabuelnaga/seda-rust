use seda_chain_adapters::ChainAdapterTrait;
use seda_runtime_sdk::Chain;
use serde::{Deserialize, Serialize};

use super::RuntimeError;

#[async_trait::async_trait]
pub trait HostAdapter: Send {
    type ChainAdapter: ChainAdapterTrait;
    fn new() -> Result<Self>
    where
        Self: Sized;

    async fn db_get(key: &str) -> Result<Option<String>, RuntimeError>;
    async fn db_set(key: &str, value: &str) -> Result<(), RuntimeError>;
    async fn http_fetch(url: &str) -> Result<String, RuntimeError>;

    async fn chain_call(
        &self,
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Option<String>, RuntimeError>;

    async fn chain_view(
        &self,
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<String, RuntimeError>;
}
