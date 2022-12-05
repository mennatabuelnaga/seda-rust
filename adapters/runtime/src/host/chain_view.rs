use std::marker::PhantomData;

use actix::prelude::*;
use seda_chain_adapters::MainChainAdapterTrait;
use seda_runtime_sdk::Chain;
use serde::{Deserialize, Serialize};

use crate::{Host, Result};

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<String>")]
pub struct ChainView<T: MainChainAdapterTrait> {
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub phantom:     PhantomData<T>,
}
impl<T: MainChainAdapterTrait> Handler<ChainView<T>> for Host {
    type Result = ResponseActFuture<Self, Result<String>>;

    fn handle(&mut self, msg: ChainView<T>, _ctx: &mut Self::Context) -> Self::Result {
        dotenv::dotenv().ok();
        let server_address = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");
        let fut = async move {
            let value = T::view2(&msg.contract_id, &msg.method_name, msg.args, &server_address)
                .await
                ?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
