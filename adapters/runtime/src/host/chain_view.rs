use std::sync::Arc;

use actix::prelude::*;
use seda_chain_adapters::MainChainAdapterTrait;

use crate::{Host, Result};

#[derive(Message)]
#[rtype(result = "Result<String>")]
pub struct ChainView<T: MainChainAdapterTrait> {
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub client:      Arc<T::Client>,
}
impl<T: MainChainAdapterTrait> Handler<ChainView<T>> for Host {
    type Result = ResponseActFuture<Self, Result<String>>;

    fn handle(&mut self, msg: ChainView<T>, _ctx: &mut Self::Context) -> Self::Result {
        dotenv::dotenv().ok();
        let fut = async move {
            let value = T::view(msg.client.clone(), &msg.contract_id, &msg.method_name, msg.args).await?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
