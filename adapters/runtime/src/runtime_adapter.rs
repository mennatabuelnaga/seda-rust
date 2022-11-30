use std::marker::PhantomData;

/// A communication layer between Actix and the runtime
use actix::prelude::*;
use seda_chain_adapters::MainChain;

// use seda_runtime::{HostAdapter, RuntimeError};
use crate::Result;
use crate::{
    host::{ChainChange, ChainView, DatabaseGet, DatabaseSet, Host, HttpFetch},
    HostAdapter,
};

pub struct RuntimeAdapter;

#[async_trait::async_trait]
impl HostAdapter for RuntimeAdapter {
    type MainChainAdapter = MainChain;

    async fn db_get(key: &str) -> Result<Option<String>> {
        let host_actor = Host::from_registry();

        let result = host_actor
            .send(DatabaseGet { key: key.to_string() })
            .await
            .unwrap()
            .unwrap();

        Ok(result)
    }

    async fn db_set(key: &str, value: &str) -> Result<()> {
        let host_actor = Host::from_registry();

        host_actor
            .send(DatabaseSet {
                key:   key.to_string(),
                value: value.to_string(),
            })
            .await
            .unwrap()
            .unwrap();

        Ok(())
    }

    async fn http_fetch(url: &str) -> Result<String> {
        let host_actor = Host::from_registry();

        let result = host_actor.send(HttpFetch { url: url.to_string() }).await.unwrap();

        Ok(result)
    }

    async fn chain_change(contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<Option<String>> {
        let host_actor = Host::from_registry();

        let result = host_actor
            .send(ChainChange::<Self::MainChainAdapter> {
                contract_id: contract_id.to_string(),
                method_name: method_name.to_string(),
                args,
                phantom: PhantomData,
            })
            .await
            .unwrap()
            .unwrap();

        Ok(result)
    }

    async fn chain_view(contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String> {
        let host_actor = Host::from_registry();

        let result = host_actor
            .send(ChainView::<Self::MainChainAdapter> {
                contract_id: contract_id.to_string(),
                method_name: method_name.to_string(),
                args,
                phantom: PhantomData,
            })
            .await
            .unwrap()
            .unwrap();

        Ok(result)
    }
}
