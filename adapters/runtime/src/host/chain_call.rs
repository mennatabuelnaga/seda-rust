use std::sync::Arc;

use actix::prelude::*;
use seda_chain_adapters::MainChainAdapterTrait;

use crate::{Host, Result};
// use seda_config::CONFIG;
#[derive(Message)]
#[rtype(result = "Result<<T as MainChainAdapterTrait>::FinalExecutionStatus>")]
pub struct ChainCall<T: MainChainAdapterTrait> {
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub deposit:     u128,
    pub client:      Arc<T::Client>,
}

impl<T: MainChainAdapterTrait> Handler<ChainCall<T>> for Host {
    type Result = ResponseActFuture<Self, Result<<T as MainChainAdapterTrait>::FinalExecutionStatus>>;

    fn handle(&mut self, msg: ChainCall<T>, _ctx: &mut Self::Context) -> Self::Result {
        let signer_acc_str = dotenv::var("SIGNER_ACCOUNT_ID").expect("SIGNER_ACCOUNT_ID not set");
        let signer_sk_str = dotenv::var("SECRET_KEY").expect("SECRET_KEY not set");
        let gas = dotenv::var("GAS").expect("GAS not set");
        let server_url = dotenv::var("NEAR_SERVER_URL").expect("NEAR_SERVER_URL not set");

        // let config = CONFIG.blocking_read();
        // let signer_acc_str =
        // config.node.as_ref().unwrap().signer_account_id.as_ref().unwrap().clone();
        // let signer_sk_str =
        // config.node.as_ref().unwrap().secret_key.as_ref().unwrap().clone();
        // let gas = config.node.as_ref().unwrap().gas.as_ref().unwrap().clone();
        // let server_url =
        // config.main_chain.as_ref().unwrap().near_server_url.as_ref().unwrap().
        // clone();

        let deposit = msg.deposit;

        let fut = async move {
            let signed_txn = T::construct_signed_tx(
                &signer_acc_str,
                &signer_sk_str,
                &msg.contract_id,
                &msg.method_name,
                msg.args,
                gas.parse()?,
                deposit,
                &server_url,
            )
            .await
            .expect("couldn't sign txn");
            let value = T::send_tx(msg.client.clone(), signed_txn).await?;

            Ok(value)
        };
        Box::pin(fut.into_actor(self))
    }
}
