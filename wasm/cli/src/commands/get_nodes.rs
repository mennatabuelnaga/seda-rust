use seda_runtime_sdk::{
    wasm::{call_self, chain_view, Promise},
    Chain,
};
use serde_json::json;

pub fn get_nodes(limit: String, offset: String) {
    // TODO: Get the node config
    let contract_id = "".to_string();
    let params = json!({
        "limit": limit,
        "offset": offset,
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
    let nodes = Promise::result(0);

    println!("node list: {:?}", nodes);
}
