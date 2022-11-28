use std::marker::PhantomData;

use actix::prelude::*;
use seda_chain_adapters::{MainChainAdapterTrait, NearMainChain};
use serde::{Deserialize, Serialize};

use crate::{Host, Result, RuntimeAdapterError};

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<Vec<u8>>")]
pub struct ChainChange<T: MainChainAdapterTrait> {
    pub signed_tx:            Vec<u8>,
    pub chain_server_address: String,
    pub phantom:              PhantomData<T>,
}

impl<T: MainChainAdapterTrait> Handler<ChainChange<T>> for Host {
    type Result = ResponseActFuture<Self, Result<Vec<u8>>>;

    fn handle(&mut self, msg: ChainChange<T>, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            let value = T::send_tx2(msg.signed_tx, &msg.chain_server_address).await.unwrap();

            Ok(value)
        };

        Box::pin(fut.into_actor(self))
    }
}
