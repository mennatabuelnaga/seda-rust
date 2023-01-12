use actix::prelude::*;
use seda_chain_adapters::{chain, Client};
use seda_runtime_sdk::Chain;

use crate::{Host, Result};

#[derive(Message)]
#[rtype(result = "Result<Vec<u8>>")]
pub struct ChainView {
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub client:      Client,
}

impl Handler<ChainView> for Host {
    type Result = ResponseActFuture<Self, Result<Vec<u8>>>;

    fn handle(&mut self, msg: ChainView, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            let value = chain::view(msg.chain, msg.client, &msg.contract_id, &msg.method_name, msg.args).await?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
