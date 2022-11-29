use std::collections::HashMap;

use futures::lock::Mutex;
use lazy_static::lazy_static;
use seda_adapters::{MainChainAdapterTrait, NearMainChain};

use super::RuntimeError;
use crate::HostAdapter;

lazy_static! {
    #[derive(Clone, Default)]
    static ref HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

}

#[derive(Clone, Default)]
pub struct HostTestAdapters {
    db: HASHMAP,
}

impl HostTestAdapters {
    async fn get(&self, key: &str) -> Result<Option<String>, RuntimeError> {
        let db = self.db.lock().await;
        let value = db.get(key);
        Ok(value.cloned())
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), RuntimeError> {
        let mut db = self.db.lock().await;
        db.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn fetch(&mut self, url: &str) -> Result<String, reqwest::Error> {
        reqwest::get(url).await.unwrap().text().await
    }

    async fn view<T: MainChainAdapterTrait>(
        &mut self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        server_addr: &str,
    ) -> Result<String, RuntimeError> {
        T::view2(contract_id, method_name, args, server_addr)
            .await
            .map_err(|err| RuntimeError::ChainInteractionsError(err.to_string()))
    }

    async fn change<T: MainChainAdapterTrait>(
        &mut self,
        signed_tx: Vec<u8>,
        server_addr: &str,
    ) -> Result<Option<String>, RuntimeError> {
        T::send_tx2(signed_tx, server_addr)
            .await
            .map_err(|err| RuntimeError::ChainInteractionsError(err.to_string()))
    }
}

pub struct RuntimeTestAdapter;

#[async_trait::async_trait]
impl HostAdapter for RuntimeTestAdapter {
    type MainChainAdapter = NearMainChain;

    async fn db_get(key: &str) -> Result<Option<String>, RuntimeError> {
        let host = HostTestAdapters::default();
        let result = host.get(key).await.expect("error getting db value");
        Ok(result)
    }

    async fn db_set(key: &str, value: &str) -> Result<(), RuntimeError> {
        let host = HostTestAdapters::default();
        host.set(key, value).await.expect("error setting db value");
        Ok(())
    }

    async fn http_fetch(url: &str) -> Result<String, RuntimeError> {
        let mut host = HostTestAdapters::default();
        let result = host.fetch(url).await.expect("error fetching http result");
        Ok(result)
    }

   
    async fn chain_view(
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        server_addr: &str,
    ) -> Result<String, RuntimeError> {
        let mut host = HostTestAdapters::default();
        let result = host
            .view::<Self::MainChainAdapter>(contract_id, method_name, args, server_addr)
            .await
            .expect("error fetching http result");
        Ok(result)
    }

    async fn chain_change(signed_tx: Vec<u8>, server_addr: &str) -> Result<Option<String>, RuntimeError> {
        let mut host = HostTestAdapters::default();
        let result = host
            .change::<Self::MainChainAdapter>(signed_tx, server_addr)
            .await
            .expect("error fetching http result");
        Ok(result)
    }
}
