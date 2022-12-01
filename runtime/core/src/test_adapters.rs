use std::collections::HashMap;

use futures::lock::Mutex;
use lazy_static::lazy_static;
use seda_chain_adapters::{MainChainAdapterTrait, NearMainChain, AnotherMainChain};

use super::RuntimeError;
use crate::{HostAdapter};
use seda_runtime_sdk::Chain;



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
    ) -> Result<String, RuntimeError> {
        dotenv::dotenv().ok();
        let server_address = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");

        T::view2(contract_id, method_name, args, &server_address)
            .await
            .map_err(|err| RuntimeError::ChainInteractionsError(err.to_string()))
    }

    async fn change<T: MainChainAdapterTrait>(
        &mut self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Option<String>, RuntimeError> {
        dotenv::dotenv().ok();
        let signer_acc_str = dotenv::var("SIGNER_ACCOUNT_ID").expect("SIGNER_ACCOUNT_ID not set");
        let signer_sk_str = dotenv::var("SECRET_KEY").expect("SECRET_KEY not set");
        let gas = dotenv::var("GAS").expect("GAS not set");
        let deposit = dotenv::var("DEPOSIT").expect("DEPOSIT not set");
        let server_url = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");

        let signed_txn = T::construct_signed_tx2(
            &signer_acc_str,
            &signer_sk_str,
            contract_id,
            method_name,
            args,
            gas.parse::<u64>().unwrap(),
            deposit.parse::<u128>().unwrap(),
            &server_url,
        ).await.expect("couldn't sign txn");
        T::send_tx2(signed_txn, &server_url)
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
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<String, RuntimeError> {
        let mut host = HostTestAdapters::default();
        if chain ==  Chain::Near {
            type MainChainAdapter = NearMainChain;
            let result = host
            .view::<MainChainAdapter>(contract_id, method_name, args)
            .await
            .expect("error fetching http result");
            Ok(result)
        }else{
            type MainChainAdapter = AnotherMainChain;
            let result = host
            .view::<MainChainAdapter>(contract_id, method_name, args)
            .await
            .expect("error fetching http result");
            Ok(result)
        }
        
        
    }

    async fn chain_change(
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,) -> Result<Option<String>, RuntimeError> {
        let mut host = HostTestAdapters::default();
        if chain ==  Chain::Near {
            type MainChainAdapter = NearMainChain;
            let result = host
            .change::<MainChainAdapter>(contract_id, method_name, args)
            .await
            .expect("error fetching http result");
        Ok(result)
        }else{
            type MainChainAdapter = AnotherMainChain;
            let result = host
            .change::<MainChainAdapter>(contract_id, method_name, args)
            .await
            .expect("error fetching http result");
        Ok(result)
        }
        
    }
}
