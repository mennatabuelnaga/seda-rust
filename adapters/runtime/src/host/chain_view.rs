use std::sync::Arc;

use actix::prelude::*;
use seda_chain_adapters::{MainChain, MainChainAdapterTrait};

use crate::{Host, Result};

#[derive(Message)]
#[rtype(result = "Result<String>")]
pub struct ChainView {
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub client:      Arc<<MainChain as MainChainAdapterTrait>::Client>,
}
impl Handler<ChainView> for Host {
    type Result = ResponseActFuture<Self, Result<String>>;

    fn handle(&mut self, msg: ChainView, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            let value = MainChain::view(msg.client.clone(), &msg.contract_id, &msg.method_name, msg.args).await?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
