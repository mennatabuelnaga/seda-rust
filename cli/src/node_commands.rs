use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
use near_primitives::views::FinalExecutionStatus;
use serde_json::json;

use crate::helpers::construct_signed_tx;

const GAS: u64 = 300_000_000_000_000;
const DEPOSIT_FOR_REGISTER_NODE: u128 = 9 * 10_u128.pow(20);

#[tokio::main]
pub async fn register_node(socket_address: String) {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let near_server_url: String = std::env::var("NEAR_SERVER_URL")
        .expect("NEAR_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let method_name = "register_node".to_string();
    let signer_acc_str: String = std::env::var("SIGNER_ACCOUNT_ID")
        .expect("SIGNER_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();
    let signer_sk_str: String = std::env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set.")
        .parse()
        .unwrap();
    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();

    let args = json!({ "socket_address": socket_address }).to_string().into_bytes();

    let signed_tx = construct_signed_tx(
        signer_acc_str,
        signer_sk_str,
        contract_id,
        method_name,
        args,
        GAS,
        DEPOSIT_FOR_REGISTER_NODE,
        near_server_url,
    )
    .await
    .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: FinalExecutionStatus = client.request("register_node", rpc_params![signed_tx]).await.unwrap();

    println!("response from server: {:?}", response);
}

#[tokio::main]
pub async fn remove_node(node_id: u64) {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let near_server_url: String = std::env::var("NEAR_SERVER_URL")
        .expect("NEAR_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let method_name = "remove_node".to_string();
    let signer_acc_str: String = std::env::var("SIGNER_ACCOUNT_ID")
        .expect("SIGNER_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();
    let signer_sk_str: String = std::env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set.")
        .parse()
        .unwrap();
    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();

    let args = json!({ "node_id": node_id.to_string() }).to_string().into_bytes();

    let signed_tx = construct_signed_tx(
        signer_acc_str,
        signer_sk_str,
        contract_id,
        method_name,
        args,
        GAS,
        0_u128,
        near_server_url,
    )
    .await
    .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: FinalExecutionStatus = client.request("remove_node", rpc_params![signed_tx]).await.unwrap();

    println!("response from server: {:?}", response);
}

#[tokio::main]
pub async fn set_node_socket_address(node_id: u64, new_socket_address: String) {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let near_server_url: String = std::env::var("NEAR_SERVER_URL")
        .expect("NEAR_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let method_name = "set_node_socket_address".to_string();
    let signer_acc_str: String = std::env::var("SIGNER_ACCOUNT_ID")
        .expect("SIGNER_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();
    let signer_sk_str: String = std::env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set.")
        .parse()
        .unwrap();
    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();

    let args = json!({ "node_id": node_id.to_string(), "new_socket_address": new_socket_address })
        .to_string()
        .into_bytes();

    let signed_tx = construct_signed_tx(
        signer_acc_str,
        signer_sk_str,
        contract_id,
        method_name,
        args,
        GAS,
        0_u128,
        near_server_url,
    )
    .await
    .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: FinalExecutionStatus = client
        .request("set_node_socket_address", rpc_params![signed_tx])
        .await
        .unwrap();

    println!("response from server: {:?}", response);
}

#[tokio::main]
pub async fn get_node_socket_address(node_id: u64) {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();
    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: String = client
        .request("get_node_socket_address", rpc_params![contract_id, node_id.to_string()])
        .await
        .unwrap();

    println!("response from server: {:?}", response);
}

#[tokio::main]
pub async fn get_node_owner(node_id: u64) {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();
    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: String = client
        .request("get_node_owner", rpc_params![contract_id, node_id.to_string()])
        .await
        .unwrap();

    println!("response from server: {:?}", response);
}
