use clap::Args;
use seda_runtime_sdk::{
    log,
    wasm::{call_self, chain_call, chain_view, Promise, CONFIG},
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
        PromiseStatus::Fulfilled(data) => {
            let data = String::from_utf8(data).expect("TODO");
            let args_string = serde_json::json!({ "data_request": data }).to_string();
            log!(Level::Debug, "Posting args: {args_string}");
            chain_call(
                Chain::Near,
                CONFIG.contract_account_id.as_str(),
                "post_data_request",
                args_string.into_bytes(),
                0,
            )
            .start()
            .then(call_self("bridge_result", vec![]));
        }
        _ => log!(Level::Debug, "Cannot bridge sub chain view failed"),
    }
}

#[no_mangle]
fn bridge_result() {
    dbg!("bridge result");
    let result = Promise::result(0);

    match result {
        PromiseStatus::Fulfilled(vec) => log!(Level::Info, "Success message: {}", String::from_utf8(vec).unwrap()),
        _ => log!(Level::Debug, "Posting bridge result to main chain failed."),
    }
}
