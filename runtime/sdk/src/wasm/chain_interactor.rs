use super::Promise;
use crate::{ChainChangeAction, ChainViewAction, PromiseAction};

pub fn chain_interactor_view(contract_id: String, method_name: String, args: Vec<u8>, server_addr: String) -> Promise {
    Promise::new(PromiseAction::ChainView(ChainViewAction {
        contract_id,
        method_name,
        args,
        server_addr,
    }))
}

pub fn chain_interactor_change(signed_tx: Vec<u8>, server_addr: String) -> Promise {
    Promise::new(PromiseAction::ChainChange(ChainChangeAction { signed_tx, server_addr }))
}
