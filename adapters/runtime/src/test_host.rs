use std::collections::HashMap;

use futures::lock::Mutex;
use lazy_static::lazy_static;
use seda_chain_adapters::{AnotherMainChain, MainChainAdapterTrait, NearMainChain};
use seda_runtime_sdk::Chain;

use crate::{HostAdapter, Result, RuntimeAdapterError};

lazy_static! {
    #[derive(Clone, Default)]
    static ref HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

}

#[derive(Clone, Default)]
pub struct HostTestAdapters;

impl HostTestAdapters {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        let db = HASHMAP.lock().await;
        let value = db.get(key);
        Ok(value.cloned())
    }

    async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut db = HASHMAP.lock().await;
        db.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn fetch(&mut self, url: &str) -> Result<String> {
        Ok(reqwest::get(url).await?.text().await?)
    }

    async fn view<T: MainChainAdapterTrait>(
        &mut self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<String> {
        dotenv::dotenv().ok();
        let server_address = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");
        T::view2(contract_id, method_name, args, &server_address)
            .await
            .map_err(|err| RuntimeAdapterError::ChainInteractionsError(err.to_string()))
    }

    async fn call<T: MainChainAdapterTrait>(
        &mut self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<Option<String>> {
        dotenv::dotenv().ok();
        let signer_acc_str = dotenv::var("SIGNER_ACCOUNT_ID").expect("SIGNER_ACCOUNT_ID not set");
        let signer_sk_str = dotenv::var("SECRET_KEY").expect("SECRET_KEY not set");
        let gas = dotenv::var("GAS").expect("GAS not set");
        let server_url = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");

        let signed_txn = T::construct_signed_tx2(
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
        T::send_tx2(signed_txn, &server_url)
            .await
            .map_err(|err| RuntimeAdapterError::ChainInteractionsError(err.to_string()))
    }
}

pub struct RuntimeTestAdapter;

#[async_trait::async_trait]
impl HostAdapter for RuntimeTestAdapter {
    type MainChainAdapter = MainChain;

    async fn db_get(key: &str) -> Result<Option<String>> {
        let host = HostTestAdapters::default();
        let result = host.get(key).await?;
        Ok(result)
    }

    async fn db_set(key: &str, value: &str) -> Result<()> {
        let host = HostTestAdapters::default();
        host.set(key, value).await?;
        Ok(())
    }

    async fn http_fetch(url: &str) -> Result<String> {
        let mut host = HostTestAdapters::default();
        let result = host.fetch(url).await?;
        Ok(result)
    }

    async fn chain_view(chain: Chain, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String> {
        let mut host = HostTestAdapters::default();
        if chain == Chain::Near {
            type MainChainAdapter = NearMainChain;
            let result = host
                .view::<MainChainAdapter>(contract_id, method_name, args)
                .await
                .expect("error fetching http result");
            Ok(result)
        } else {
            type MainChainAdapter = AnotherMainChain;
            let result = host
                .view::<MainChainAdapter>(contract_id, method_name, args)
                .await
                .expect("error fetching http result");
            Ok(result)
        }
    }

    async fn chain_call(
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<Option<String>> {
        let mut host = HostTestAdapters::default();

        if chain == Chain::Near {
            type MainChainAdapter = NearMainChain;
            let result = host
                .call::<MainChainAdapter>(contract_id, method_name, args, deposit)
                .await
                .expect("error fetching http result");
            Ok(result)
        } else {
            type MainChainAdapter = AnotherMainChain;
            let result = host
                .call::<MainChainAdapter>(contract_id, method_name, args, deposit)
                .await
                .expect("error fetching http result");
            Ok(result)
        }
    }
}
