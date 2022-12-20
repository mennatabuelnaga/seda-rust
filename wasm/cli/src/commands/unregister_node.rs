use seda_runtime_sdk::{
    wasm::{call_self, chain_call, Promise},
    Chain,
    PromiseStatus,
};
use serde_json::json;

pub fn unregister_node(node_id: u64) {
    // TODO: Get the node config
    let contract_id = "mc.mennat0.testnet".to_string();
    let params = json!({
        "node_id": node_id.to_string(),
    })
    .to_string()
    .into_bytes();

    println!("Sending params");
    let deposit = "0".to_string();

    chain_call(Chain::Near, contract_id, "unregister_node".to_string(), params, deposit)
        .start()
        .then(call_self("unregister_node_step_1", vec![]));
}

#[no_mangle]
pub fn unregister_node_step_1() {
    let result = Promise::result(0);
    let status: String = match result {
        PromiseStatus::Fulfilled(_) => "node unregistered".to_string(),
        _ => "Promise failed..".to_string(),
    };

    println!("status: {:?}", status);
}
