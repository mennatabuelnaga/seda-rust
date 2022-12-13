use std::{collections::HashMap, sync::Arc};

use futures::lock::Mutex;
use lazy_static::lazy_static;
use seda_chain_adapters::{MainChain, MainChainAdapterTrait};
use seda_config::CONFIG;

use crate::{HostAdapter, Result, RuntimeAdapterError};

lazy_static! {
    #[derive(Clone, Default)]
    static ref HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

}

pub struct RuntimeTestAdapter {
    pub client: Arc<<MainChain as MainChainAdapterTrait>::Client>,
}

#[async_trait::async_trait]
impl HostAdapter for RuntimeTestAdapter {
    async fn new() -> Result<Self> {
        let config = CONFIG.read().await;
        // Safe to unwrap here, it's already been checked.
        let main_chain_config = config.main_chain.as_ref().unwrap();

        Ok(Self {
            client: Arc::new(MainChain::new_client(main_chain_config)?),
        })
    }

    async fn db_get(&self, key: &str) -> Result<Option<String>> {
        let db = HASHMAP.lock().await;
        let value = db.get(key);
        Ok(value.cloned())
    }

    async fn db_set(&self, key: &str, value: &str) -> Result<()> {
        let mut db = HASHMAP.lock().await;
        db.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn http_fetch(&self, url: &str) -> Result<String> {
        Ok(reqwest::get(url).await?.text().await?)
    }

    async fn chain_view(&self, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String> {
        MainChain::view(self.client.clone(), contract_id, method_name, args)
            .await
            .map_err(|err| RuntimeAdapterError::ChainInteractionsError(err.to_string()))
    }

    async fn chain_call(
        &self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<<MainChain as MainChainAdapterTrait>::FinalExecutionStatus> {
        let config = CONFIG.read().await;
        let node_config = config.node.as_ref().unwrap();
        let signer_acc_str = node_config.signer_account_id.as_ref().unwrap();
        let signer_sk_str = node_config.secret_key.as_ref().unwrap();
        let gas = node_config.gas.as_ref().unwrap();
        let server_url = config.main_chain.as_ref().unwrap().chain_server_url.as_ref().unwrap();

        let signed_txn = MainChain::construct_signed_tx(
            signer_acc_str,
            signer_sk_str,
            contract_id,
            method_name,
            args,
            gas.parse::<u64>()?,
            deposit,
            server_url,
        )
        .await
        .expect("couldn't sign txn");
        MainChain::send_tx(self.client.clone(), signed_txn)
            .await
            .map_err(|err| RuntimeAdapterError::ChainInteractionsError(err.to_string()))
    }
}
