use std::sync::Arc;

/// A communication layer between Actix and the runtime
use actix::prelude::*;
use seda_chain_adapters::{MainChain, MainChainAdapterTrait};
use seda_config::CONFIG;

use crate::{ChainCall, ChainView, DatabaseGet, DatabaseSet, Host, HostAdapter, HttpFetch, Result};
pub struct RuntimeAdapter {
    pub client: Arc<<MainChain as MainChainAdapterTrait>::Client>,
}

#[async_trait::async_trait]
impl HostAdapter for RuntimeAdapter {
    async fn new() -> Result<Self> {
        let config = CONFIG.read().await;
        // Safe to unwrap here, it's already been checked.
        let main_chain_config = config.main_chain.as_ref().unwrap();

        Ok(Self {
            client: Arc::new(MainChain::new_client(main_chain_config)?),
        })
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
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        deposit: u128,
    ) -> Result<<MainChain as MainChainAdapterTrait>::FinalExecutionStatus> {
        let host_actor = Host::from_registry();
        let result = host_actor
            .send(ChainCall::<MainChain> {
                contract_id: contract_id.to_string(),
                method_name: method_name.to_string(),
                args,
                deposit,
                client: self.client.clone(),
            })
            .await??;

        Ok(result)
    }

    async fn chain_view(&self, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String> {
        let host_actor = Host::from_registry();
        let result = host_actor
            .send(ChainView::<MainChain> {
                contract_id: contract_id.to_string(),
                method_name: method_name.to_string(),
                args,
                client: self.client.clone(),
            })
            .await??;

        Ok(result)
    }
}
