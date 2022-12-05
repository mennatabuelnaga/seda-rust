use serde::{Deserialize, Serialize};

use super::Promise;
use crate::{Chain, ChainChangeAction, ChainViewAction, PromiseAction};

pub fn chain_view(chain: Chain, contract_id: String, method_name: String, args: Vec<u8>) -> Promise {
    Promise::new(PromiseAction::ChainView(ChainViewAction {
        chain,
        contract_id,
        method_name,
        args,
    }))
}

pub fn chain_call(chain: Chain, contract_id: String, method_name: String, args: Vec<u8>) -> Promise {
    Promise::new(PromiseAction::ChainCall(ChainChangeAction {
        chain,
        contract_id,
        method_name,
        args,
    }))
}
