use std::sync::Arc;

use actix::prelude::*;
use seda_adapters::MainChainAdapterTrait;
use seda_runtime_sdk::Chain;

use super::Host;
use crate::NodeError;
use seda_config::CONFIG;

#[derive(Message)]
#[rtype(result = "Result<Option<String>, NodeError>")]
pub struct ChainCall<T: MainChainAdapterTrait> {
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub client:     Arc<T::Client>,
}

impl<T: MainChainAdapterTrait> Handler<ChainCall<T>> for Host {
    type Result = ResponseActFuture<Self, Result<Option<String>, NodeError>>;

    fn handle(&mut self, msg: ChainCall<T>, _ctx: &mut Self::Context) -> Self::Result {
        let deposit = msg.deposit;



        let fut = async move {
            let config = CONFIG.read().await;
            let node_config = config.node.as_ref().unwrap();
            let signer_acc_str = node_config.signer_account_id.as_ref().unwrap();
            let signer_sk_str = node_config.secret_key.as_ref().unwrap();
            let gas = node_config.gas.as_ref().unwrap();
            let server_url = config.main_chain.as_ref().unwrap().chain_server_url.as_ref().unwrap();
            let signed_txn = T::construct_signed_tx(
                &signer_acc_str,
                &signer_sk_str,
                &msg.contract_id,
                &msg.method_name,
                msg.args,
                gas.parse()?,
                deposit.parse()?,
                &server_url,
            )
            .await
            .expect("couldn't sign txn");
            let value = T::send_tx(signed_txn, &server_url).await?;

            Ok(value)
        };

        Box::pin(fut.into_actor(self))
    }
}
