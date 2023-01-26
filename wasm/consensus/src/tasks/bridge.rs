use clap::Args;
use seda_runtime_sdk::{
    log,
    wasm::{call_self, chain_call, chain_view, memory_read, memory_write, Promise, CONFIG},
    Chain,
    Level,
    PromiseStatus,
};

#[derive(Debug, Args)]
pub struct Bridge {
    chain:       Chain,
    contract_id: String,
    method_name: String,
    deposit:     u128,
    args:        String,
}

impl Bridge {
    pub fn handle(self) {
        // we have a method to auto convert to bytes in a trait in runtime.
        // it should be moved to the sdk
        memory_write("bridge_deposit", self.deposit.to_le_bytes().to_vec());
        chain_view(self.chain, self.contract_id, self.method_name, self.args.into_bytes())
            .start()
            .then(call_self("bridge", vec![]));
    }
}

// Oh right wasmer doesn't have strings... or u128s so passing deposit as an arg
// may not be good
#[no_mangle]
fn bridge() {
    let result = Promise::result(0);
    let deposit_bytes = memory_read("bridge_deposit");
    let deposit = u128::from_le_bytes(
        deposit_bytes
            .try_into()
            .expect("This method is in a nice trait in runtime."),
    );
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
                deposit,
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
    println!("{{\"status\": \"success\"}}");
    match result {
        PromiseStatus::Fulfilled(vec) => log!(Level::Debug, "Success message: {}", String::from_utf8(vec).unwrap()),
        _ => log!(Level::Error, "Posting bridge result to main chain failed."),
    }
}
