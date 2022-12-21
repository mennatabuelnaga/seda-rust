use parse_display::{Display, FromStr};
use seda_config::CONFIG;
use seda_runtime_sdk::{
    wasm::{call_self, chain_view, Promise},
    Chain,
    PromiseStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn update_node(node_id: u64, command: UpdateNode) {
    // TODO: Get the node config
    let contract_id = "mc.mennat0.testnet".to_string();
    let params = json!({
        "node_id": node_id.to_string(),
        "command": command.to_string()
    })
    .to_string()
    .into_bytes();

    println!("Sending params");

    chain_view(Chain::Near, contract_id, "update_node".to_string(), params)
        .start()
        .then(call_self("update_node_step_1", vec![]));
}

#[no_mangle]
pub fn update_node_step_1() {
    let result = Promise::result(0);
    let status: String = match result {
        PromiseStatus::Fulfilled(_) => "node updated".to_string(),
        _ => "Promise failed..".to_string(),
    };

    println!("status: {:?}", status);
}

/// Update node commands
#[derive(Deserialize, Serialize, Clone, Display, FromStr, Debug)]
pub enum UpdateNode {
    AcceptOwnership,
    #[display("SetPendingOwner{0}")]
    SetPendingOwner(String),
    #[display("SetSocketAddress({0})")]
    SetSocketAddress(String),
}
