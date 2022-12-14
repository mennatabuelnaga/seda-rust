use std::sync::Arc;

use actix::prelude::*;
use seda_chain_adapters::{MainChain, MainChainAdapterTrait};
use seda_config::CONFIG;

use crate::{Host, Result};
#[derive(Message)]
#[rtype(result = "Result<<MainChain as MainChainAdapterTrait>::FinalExecutionStatus>")]
pub struct ChainCall {
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub deposit:     u128,
    pub client:      Arc<<MainChain as MainChainAdapterTrait>::Client>,
}

impl Handler<ChainCall> for Host {
    type Result = ResponseActFuture<Self, Result<<MainChain as MainChainAdapterTrait>::FinalExecutionStatus>>;

    fn handle(&mut self, msg: ChainCall, _ctx: &mut Self::Context) -> Self::Result {
        let deposit = msg.deposit;
        let fut = async move {
            let config = CONFIG.read().await;
            let node_config = config.node.as_ref().unwrap();
            let signer_acc_str = node_config.signer_account_id.as_ref().unwrap();
            let signer_sk_str = node_config.secret_key.as_ref().unwrap();
            let gas = node_config.gas.as_ref().unwrap();
            let server_url = config.main_chain.as_ref().unwrap().chain_server_url.as_ref().unwrap();

            let signed_txn = MainChain::construct_signed_tx(
                signer_acc_str,
                signer_sk_str,
                &msg.contract_id,
                &msg.method_name,
                msg.args,
                gas.parse()?,
                deposit,
                server_url,
            )
            .await?;
            let value = MainChain::send_tx(msg.client.clone(), signed_txn).await?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
