use std::marker::PhantomData;

use actix::prelude::*;
use seda_chain_adapters::MainChainAdapterTrait;
use serde::{Deserialize, Serialize};

use crate::{Host, Result};

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<Option<String>>")]
pub struct ChainChange<T: MainChainAdapterTrait> {
    pub contract_id:          String,
    pub method_name:          String,
    pub args:                 Vec<u8>,
    pub phantom:              PhantomData<T>,
}

impl<T: MainChainAdapterTrait> Handler<ChainChange<T>> for Host {
    type Result = ResponseActFuture<Self, Result<Option<String>>>;

    fn handle(&mut self, msg: ChainChange<T>, _ctx: &mut Self::Context) -> Self::Result {
        let signer_acc_str = dotenv::var("SIGNER_ACCOUNT_ID").expect("SIGNER_ACCOUNT_ID not set");
        let signer_sk_str = dotenv::var("SECRET_KEY").expect("SECRET_KEY not set");
        let gas = dotenv::var("GAS").expect("GAS not set");
        let deposit = dotenv::var("DEPOSIT").expect("DEPOSIT not set");
        let server_url = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");

        
        
        let fut = async move {
            let signed_txn = T::construct_signed_tx2(
                &signer_acc_str,
                &signer_sk_str,
                &msg.contract_id,
                &msg.method_name,
                msg.args,
                gas.parse().unwrap(),
                deposit.parse().unwrap(),
                &server_url,
            ).await.expect("couldn't sign txn");
            let value = T::send_tx2(signed_txn, &server_url).await.unwrap();

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
