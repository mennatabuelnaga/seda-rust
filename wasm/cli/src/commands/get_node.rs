use borsh::{BorshDeserialize, BorshSerialize};
use seda_config::CONFIG;
use seda_runtime_sdk::{
    wasm::{call_self, chain_view, Promise},
    Chain,
    PromiseStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, json};

// cargo run -- -c near cli get-node 1
pub fn get_node(node_id: u64) {
    // TODO: Get the node config
    let contract_id = "mc.mennat0.testnet".to_string();

    // let config = CONFIG.blocking_read();
    // let contract_id = &config.node.contract_account_id;
    let params = json!({
        "node_id": node_id.to_string()
    })
    .to_string()
    .into_bytes();

    println!("Sending params");

    chain_view(Chain::Near, contract_id.to_string(), "get_node".to_string(), params)
        .start()
        .then(call_self("get_node_step_1", vec![]));
}

#[no_mangle]
pub fn get_node_step_1() {
    let result = Promise::result(0);
    let node: Option<Node> = match result {
        PromiseStatus::Fulfilled(vec) => from_slice::<Option<Node>>(&vec).unwrap(),
        // _ => vec!["Promise failed..".to_string()],
        _ => None,
    };

    println!("node: {:?}", node);
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct Node {
    pub owner:          String,
    pub pending_owner:  Option<String>,
    pub socket_address: String, // ip address and port
}
