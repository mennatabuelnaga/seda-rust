use std::sync::Arc;

use actix::prelude::*;
use seda_adapters::MainChainAdapterTrait;
use seda_runtime_sdk::Chain;

use super::Host;
use crate::Result;

#[derive(Message)]
#[rtype(result = "Result<String>")]
pub struct ChainView<T: MainChainAdapterTrait> {
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub client:      Arc<T::Client>,
}
impl<T: MainChainAdapterTrait> Handler<ChainView<T>> for Host {
    type Result = ResponseActFuture<Self, Result<String>>;

    fn handle(&mut self, msg: ChainView<T>, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            let value = T::view(msg.client.clone(), &msg.contract_id, &msg.method_name, msg.args)
                .await
                .unwrap();

            Ok(value)
        };

        Box::pin(fut.into_actor(self))
    }
}
