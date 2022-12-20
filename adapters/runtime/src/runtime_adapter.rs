use std::sync::Arc;

/// A communication layer between Actix and the runtime
use actix::prelude::*;
use seda_chain_adapters::{AnotherMainChain, Client, MainChainAdapterTrait, NearMainChain};
use seda_config::CONFIG;
use seda_runtime_sdk::Chain;

use crate::{ChainCall, ChainView, DatabaseGet, DatabaseSet, Host, HostAdapter, HttpFetch, Result};
pub struct RuntimeAdapter {
    pub another_client: Client,
    pub near_client:    Client,
}

#[async_trait::async_trait]
impl HostAdapter for RuntimeAdapter {
    async fn new() -> Result<Self> {
        let config = CONFIG.read().await;
        // Safe to unwrap here, it's already been checked.
        let config = config.as_ref();
        Ok(Self {
            another_client: Client::Another(Arc::new(AnotherMainChain::new_client(&config.another_chain)?)),
            near_client:    Client::Near(Arc::new(NearMainChain::new_client(&config.near_chain)?)),
        })
    }

    fn select_client_from_chain(&self, chain: Chain) -> Client {
        match chain {
            Chain::Another => self.another_client.clone(),
            Chain::Near => self.near_client.clone(),
        }
    }

    async fn db_get(&self, key: &str) -> Result<Option<String>> {
        let host_actor = Host::from_registry();

        let result = host_actor.send(DatabaseGet { key: key.to_string() }).await??;

        Ok(result)
    }

    async fn db_set(&self, key: &str, value: &str) -> Result<()> {
        let host_actor = Host::from_registry();

        host_actor
            .send(DatabaseSet {
                key:   key.to_string(),
                value: value.to_string(),
            })
            .await??;

        Ok(())
    }

    async fn http_fetch(&self, url: &str) -> Result<String> {
        let host_actor = Host::from_registry();

        let result = host_actor.send(HttpFetch { url: url.to_string() }).await?;

        Ok(result)
    }

    async fn chain_call(
        &self,
        chain: Chain,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<Vec<u8>> {
        let host_actor = Host::from_registry();
        let client = self.select_client_from_chain(chain);
        let result = host_actor
            .send(ChainCall {
                chain,
                contract_id: contract_id.to_string(),
                method_name: method_name.to_string(),
                args,
                deposit,
                client,
            })
            .await??;

        Ok(result)
    }

    async fn chain_view(&self, chain: Chain, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<Vec<u8>> {
        let host_actor = Host::from_registry();
        let client = self.select_client_from_chain(chain);
        let result = host_actor
            .send(ChainView {
                chain,
                contract_id: contract_id.to_string(),
                method_name: method_name.to_string(),
                args,
                client,
            })
            .await??;

        Ok(result)
    }
}
