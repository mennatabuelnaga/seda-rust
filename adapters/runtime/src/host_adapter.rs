use seda_chain_adapters::MainChainAdapterTrait;

use crate::Result;

// #[async_trait::async_trait]
// pub trait HostAdapter: Send {
//     async fn db_get(key: &str) -> Result<Option<String>>;
//     async fn db_set(key: &str, value: &str) -> Result<()>;
//     async fn http_fetch(url: &str) -> Result<String>;
// }



#[async_trait::async_trait]
pub trait HostAdapter: Send {
    type MainChainAdapter: MainChainAdapterTrait;

    async fn db_get(key: &str) -> Result<Option<String>>;
    async fn db_set(key: &str, value: &str) -> Result<()>;
    async fn http_fetch(url: &str) -> Result<String>;
    
    async fn chain_change(signed_tx: Vec<u8>, server_addr: &str) -> Result<Vec<u8>>;

    async fn chain_view(
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        server_addr: &str,
    ) -> Result<String>;
}
