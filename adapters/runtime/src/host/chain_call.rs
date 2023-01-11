use actix::prelude::*;
use seda_chain_adapters::{chain, Client};
use seda_runtime_sdk::Chain;

use crate::{Host, Result};
#[derive(Message)]
#[rtype(result = "Result<Vec<u8>>")]
pub struct ChainCall {
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub deposit:     u128,
    pub client:      Client,
}

impl Handler<ChainCall> for Host {
    type Result = ResponseActFuture<Self, Result<Vec<u8>>>;

    fn handle(&mut self, msg: ChainCall, _ctx: &mut Self::Context) -> Self::Result {
        let deposit = msg.deposit;
        // let fut = async move {
        //     let config = CONFIG.read().await;
        //     let node_config = &config.node;
        //     let signer_acc_str = &node_config.signer_account_id;
        //     let signer_sk_str = &node_config.secret_key;
        //     let gas = &node_config.gas;
        //     let server_url = match msg.chain {
        //         Chain::Another => &config.another_chain.chain_rpc_url,
        //         Chain::Near => &config.near_chain.chain_rpc_url,
        //     };

        //     let signed_txn = chain::construct_signed_tx(
        //         msg.chain,
        //         signer_acc_str,
        //         signer_sk_str,
        //         &msg.contract_id,
        //         &msg.method_name,
        //         msg.args,
        //         gas.parse()?,
        //         deposit,
        //         server_url,
        //     )
        //     .await?;
        //     let value = chain::send_tx(msg.chain, msg.client, &signed_txn).await?;

        //     Ok(value)
        // };
        todo!()
        // Box::pin(fut.into_actor(self))
    }
}
