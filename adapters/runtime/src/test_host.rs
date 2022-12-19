use std::{collections::HashMap, sync::Arc};

use futures::lock::Mutex;
use lazy_static::lazy_static;
use seda_chain_adapters::{chain, AnotherMainChain, Client, MainChainAdapterTrait, NearMainChain};
use seda_config::CONFIG;
use seda_runtime_sdk::{Chain, Event};

use crate::{HostAdapter, Result};

lazy_static! {
    #[derive(Clone, Default)]
    static ref HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

}

pub struct RuntimeTestAdapter {
    pub another_client: Client,
    pub near_client:    Client,
}

#[async_trait::async_trait]
impl HostAdapter for RuntimeTestAdapter {
    async fn new() -> Result<Self> {
        let config = CONFIG.read().await;
        // Safe to unwrap here, it's already been checked.
        let config = config.as_ref();
        Ok(Self {
            another_client: Client::Another(Arc::new(AnotherMainChain::new_client(
                config.another_chain.as_ref().expect("TODO clean up when de-optioning."),
            )?)),
            near_client:    Client::Near(Arc::new(NearMainChain::new_client(
                config.near_chain.as_ref().expect("TODO clean up when de-optioning."),
            )?)),
        })
    }

    fn select_client_from_chain(&self, chain: Chain) -> Client {
        match chain {
            Chain::Another => self.another_client.clone(),
            Chain::Near => self.near_client.clone(),
        }
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

    async fn chain_view(&self, chain: Chain, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String> {
        let client = self.select_client_from_chain(chain);
        Ok(chain::view(chain, client, contract_id, method_name, args).await?)
    }

    async fn chain_call(
        &self,
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<Vec<u8>> {
        let config = CONFIG.read().await;
        let node_config = config.node.as_ref().unwrap();
        let signer_acc_str = node_config.signer_account_id.as_ref().unwrap();
        let signer_sk_str = node_config.secret_key.as_ref().unwrap();
        let gas = node_config.gas.as_ref().unwrap();
        let server_url = match chain {
            Chain::Another => config.another_chain.as_ref().unwrap().chain_rpc_url.as_ref().unwrap(),
            Chain::Near => config.near_chain.as_ref().unwrap().chain_rpc_url.as_ref().unwrap(),
        };

        let signed_txn = chain::construct_signed_tx(
            chain,
            signer_acc_str,
            signer_sk_str,
            contract_id,
            method_name,
            args,
            gas.parse::<u64>()?,
            deposit,
            server_url,
        )
        .await?;
        let client = self.select_client_from_chain(chain);
        Ok(chain::send_tx(chain, client, &signed_txn).await?)
    }

    async fn trigger_event(event: Event) -> Result<()> {
        dbg!(event);
        Ok(())
    }
}
