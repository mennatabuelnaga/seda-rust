use seda_config::CONFIG;
use seda_runtime_sdk::{
    wasm::{call_self, chain_call, Promise},
    Chain,
    PromiseStatus,
};
use serde_json::{from_slice, json};

// cargo run -- -c near cli register-node 127.0.0.1:8080 870000000000000000000
pub fn register_node(socket_address: String, deposit: String) {
    // TODO: Get the node config
    let contract_id = "mc.mennat0.testnet".to_string();
    let params = json!({
        "socket_address": socket_address,
    })
    .to_string()
    .into_bytes();

    println!("Sending params");

    chain_call(Chain::Near, contract_id, "register_node".to_string(), params, deposit)
        .start()
        .then(call_self("register_node_step_1", vec![]));
}

#[no_mangle]
pub fn register_node_step_1() {
    let result = Promise::result(0);
    let node_id: String = match result {
        PromiseStatus::Fulfilled(vec) => from_slice::<String>(&vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };

    println!("registered node id: {:?}", node_id);
}
