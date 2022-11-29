use std::marker::PhantomData;

use actix::prelude::*;
use seda_adapters::MainChainAdapterTrait;
use serde::{Deserialize, Serialize};

use super::Host;
use crate::NodeError;

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<String, NodeError>")]
pub struct ChainView<T: MainChainAdapterTrait> {
    pub contract_id:          String,
    pub method_name:          String,
    pub args:                 Vec<u8>,
    pub chain_server_address: String,
    pub phantom:              PhantomData<T>,
}
impl<T: MainChainAdapterTrait> Handler<ChainView<T>> for Host {
    type Result = ResponseActFuture<Self, Result<String, NodeError>>;

    fn handle(&mut self, msg: ChainView<T>, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            let value = T::view2(&msg.contract_id, &msg.method_name, msg.args, &msg.chain_server_address)
                .await
                .unwrap();

            Ok(value)
        };

        Box::pin(fut.into_actor(self))
    }
}
