use super::Promise;
use crate::{ChainCallAction, ChainViewAction, PromiseAction};

pub fn chain_view(contract_id: String, method_name: String, args: Vec<u8>) -> Promise {
    Promise::new(PromiseAction::ChainView(ChainViewAction {
        contract_id,
        method_name,
        args,
    }))
}

pub fn chain_call(contract_id: String, method_name: String, args: Vec<u8>, deposit: String) -> Promise {
    Promise::new(PromiseAction::ChainCall(ChainCallAction {
        contract_id,
        method_name,
        args,
        deposit,
    }))
}
