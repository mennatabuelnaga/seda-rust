use seda_runtime_sdk::{
    wasm::{call_self, chain_view, Promise},
    Chain,
    PromiseStatus,
};
use serde_json::{from_slice, json};

use crate::commands::Node;

pub fn get_node(node_id: u64) {
    // TODO: Get the node config
    let contract_id = "mc.mennat0.testnet".to_string();
    let params = json!({
        "node_id": node_id.to_string()
    })
    .to_string()
    .into_bytes();

    println!("Sending params");

    chain_view(Chain::Near, contract_id, "get_node".to_string(), params)
        .start()
        .then(call_self("get_node_step_1", vec![]));
}

#[no_mangle]
pub fn get_node_step_1() {
    let result = Promise::result(0);
    let node: Option<Node> = match result {
        PromiseStatus::Fulfilled(vec) => from_slice::<Option<Node>>(&vec).unwrap(),
        _ => None,
    };

    println!("node: {:?}", node);
}
