use borsh::{BorshDeserialize, BorshSerialize};
use seda_runtime_sdk::{
    wasm::{call_self, chain_view, Promise},
    Chain,
    PromiseStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, json};

pub fn get_nodes(limit: u64, offset: u64) {
    // TODO: Get the node config
    let contract_id = "mc.mennat0.testnet".to_string();
    let params = json!({
        "limit": limit.to_string(),
        "offset": offset.to_string(),
    })
    .to_string()
    .into_bytes();

    println!("Sending params");

    chain_view(Chain::Near, contract_id, "get_nodes".to_string(), params)
        .start()
        .then(call_self("get_nodes_step_1", vec![]));
}

#[no_mangle]
pub fn get_nodes_step_1() {
    let result = Promise::result(0);
    let nodes: Vec<Node> = match result {
        PromiseStatus::Fulfilled(vec) => from_slice::<Vec<Node>>(&vec).unwrap(),
        _ => vec![Node {
            owner:          "".to_string(),
            pending_owner:  None,
            socket_address: "".to_string(),
        }],
    };

    println!("node list: {:#?}", nodes);
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct Node {
    pub owner:          String,
    pub pending_owner:  Option<String>,
    pub socket_address: String, // ip address and port
}
