// use seda_config::CONFIG;
use seda_runtime_sdk::{
    wasm::{call_self, chain_view, Promise, db_set},
    Chain,
    PromiseStatus,
};
use serde_json::json;

pub fn get_node_owner(node_id: String) {
    // let config = CONFIG.blocking_read();
    // let config = CONFIG.read().await;
    // let contract_id = &config.node.contract_account_id;
    let contract_id = "mc.mennat0.testnet";
    let params = json!({
        "node_id": node_id,
    })
    .to_string()
    .into_bytes();


    chain_view(Chain::Near, contract_id.to_string(), "get_node_owner".to_string(), params)
        .start()
        .then(call_self("get_node_owner_success", vec![]));


}

#[no_mangle]
pub fn get_node_owner_success() {
    let result = Promise::result(0);
    let value_to_store: String = match result {
        PromiseStatus::Fulfilled(vec) => String::from_utf8(vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };
    println!("*********Value: {value_to_store}");
    db_set("get_node_owner_result", &value_to_store).start();
}
