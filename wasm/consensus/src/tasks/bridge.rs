use clap::Args;
use seda_runtime_sdk::{
    log,
    wasm::{call_self, chain_call, chain_view, Promise},
    Chain,
    Level,
    PromiseStatus,
};

#[derive(Debug, Args)]
pub struct Bridge {
    chain:       Chain,
    contract_id: String,
    method_name: String,
    args:        String,
}

impl Bridge {
    pub fn handle(self) {
        chain_view(self.chain, self.contract_id, self.method_name, self.args.into_bytes())
            .start()
            .then(call_self("bridge", vec![]));
    }
}

#[no_mangle]
fn bridge() {
    let result = Promise::result(0);

    match result {
        PromiseStatus::Fulfilled(args) => {
            chain_call(
                Chain::Near,
                // node_config.contract_account_id?
                "mc.mennat0.testnet",
                "post_data_request",
                args,
                // node_config.deposit?
                0,
            )
            .start()
            .then(call_self("bridge_result", vec![]));
        }
        _ => log!(Level::Error, "Cannot bridge sub chain view failed"),
    }
}

#[no_mangle]
fn bridge_result() {
    let result = Promise::result(0);

    match result {
        PromiseStatus::Fulfilled(vec) => log!(Level::Info, "Success message: {}", String::from_utf8(vec).unwrap()),
        _ => log!(Level::Error, "Posting bridge result to main chain failed."),
    }
}
