use super::Promise;
use crate::{Chain, ChainCallAction, ChainViewAction, PromiseAction};

pub fn chain_view<S: ToString>(chain: Chain, contract_id: S, method_name: S, args: Vec<u8>) -> Promise {
    Promise::new(PromiseAction::ChainView(ChainViewAction {
        chain,
        contract_id: contract_id.to_string(),
        method_name: method_name.to_string(),
        args,
    }))
}

pub fn chain_call<S: ToString>(chain: Chain, contract_id: S, method_name: S, args: Vec<u8>, deposit: u128) -> Promise {
    Promise::new(PromiseAction::ChainCall(ChainCallAction {
        chain,
        contract_id: contract_id.to_string(),
        method_name: method_name.to_string(),
        args,
        deposit,
    }))
}
