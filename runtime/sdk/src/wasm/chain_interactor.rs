use super::Promise;
use crate::{ChainChangeAction, ChainViewAction, PromiseAction};

pub fn chain_view(contract_id: String, method_name: String, args: Vec<u8>) -> Promise {
    Promise::new(PromiseAction::ChainView(ChainViewAction {
        contract_id,
        method_name,
        args,
    }))
}

pub fn chain_change(contract_id: String, method_name: String, args: Vec<u8>) -> Promise {
    Promise::new(PromiseAction::ChainChange(ChainChangeAction { contract_id,
        method_name,
        args, }))
}
