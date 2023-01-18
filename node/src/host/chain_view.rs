use actix::prelude::*;
use seda_chains::{chain, Client};
use seda_runtime::HostAdapter;
use seda_runtime_sdk::Chain;

use crate::{Host, Result};

#[derive(Message)]
#[rtype(result = "Result<String>")]
pub struct ChainView {
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub client:      Client,
}

impl<HA: HostAdapter> Handler<ChainView> for Host<HA> {
    type Result = ResponseActFuture<Self, Result<String>>;

    fn handle(&mut self, msg: ChainView, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            let value = chain::view(msg.chain, msg.client, &msg.contract_id, &msg.method_name, msg.args).await?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
