use actix::prelude::*;
use seda_chains::{chain, Client};
use seda_config::{ChainConfigs, NodeConfig};
use seda_runtime_sdk::Chain;

use crate::{Host, Result};
#[derive(Message)]
#[rtype(result = "Result<Vec<u8>>")]
pub struct ChainCall {
    pub chain:         Chain,
    pub contract_id:   String,
    pub method_name:   String,
    pub args:          Vec<u8>,
    pub client:        Client,
    pub deposit:       u128,
    pub node_config:   NodeConfig,
    pub chains_config: ChainConfigs,
}

impl Handler<ChainCall> for Host {
    type Result = ResponseActFuture<Self, Result<Vec<u8>>>;

    fn handle(&mut self, msg: ChainCall, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            let server_url = match msg.chain {
                Chain::Another => &msg.chains_config.another.chain_rpc_url,
                Chain::Near => &msg.chains_config.near.chain_rpc_url,
            };

            let signed_txn = chain::construct_signed_tx(
                msg.chain,
                &msg.node_config.signer_account_id,
                &msg.node_config.secret_key,
                &msg.contract_id,
                &msg.method_name,
                msg.args,
                msg.node_config.gas,
                msg.node_config.deposit,
                server_url,
            )
            .await?;
            let value = chain::send_tx(msg.chain, msg.client, &signed_txn).await?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
