use std::{collections::HashMap, sync::Arc};

use futures::lock::Mutex;
use lazy_static::lazy_static;
use seda_chain_adapters::MainChainAdapterTrait;
use seda_config::CONFIG;

use crate::{HostAdapter, Result, RuntimeAdapterError};

lazy_static! {
    #[derive(Clone, Default)]
    static ref HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

}

pub struct RuntimeTestAdapter<T: MainChainAdapterTrait> {
    pub client: Arc<T::Client>,
}

#[async_trait::async_trait]
impl<T: MainChainAdapterTrait> HostAdapter for RuntimeTestAdapter<T> {
    type MainChainAdapter = T;

    fn new() -> Result<Self> {
        let config = CONFIG.blocking_read();
        // Safe to unwrap here, it's already been checked.
        let main_chain_config = config.main_chain.as_ref().unwrap();

        Ok(Self {
            client: Arc::new(Self::MainChainAdapter::new_client(main_chain_config)?),
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
        dotenv::dotenv().ok();
        T::view(self.client.clone(), contract_id, method_name, args)
            .await
            .map_err(|err| RuntimeAdapterError::ChainInteractionsError(err.to_string()))
    }

    async fn chain_call(
        &self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<<Self::MainChainAdapter as MainChainAdapterTrait>::FinalExecutionStatus> {
        dotenv::dotenv().ok();
        let signer_acc_str = dotenv::var("SIGNER_ACCOUNT_ID").expect("SIGNER_ACCOUNT_ID not set");
        let signer_sk_str = dotenv::var("SECRET_KEY").expect("SECRET_KEY not set");
        let gas = dotenv::var("GAS").expect("GAS not set");
        let server_url = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");

        let signed_txn = T::construct_signed_tx(
            &signer_acc_str,
            &signer_sk_str,
            contract_id,
            method_name,
            args,
            gas.parse::<u64>()?,
            deposit,
            &server_url,
        )
        .await
        .expect("couldn't sign txn");
        T::send_tx(self.client.clone(), signed_txn)
            .await
            .map_err(|err| RuntimeAdapterError::ChainInteractionsError(err.to_string()))
    }
}
