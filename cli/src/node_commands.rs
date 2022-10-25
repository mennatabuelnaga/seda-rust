use crate::helpers::construct_signed_tx;
use near_primitives::views::FinalExecutionStatus;
use serde_json::{json, Number};
use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};

#[tokio::main]
pub async fn register() {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let near_server_url: String = std::env::var("NEAR_SERVER_URL")
        .expect("NEAR_SERVER_URL must be set.")
        .parse()
        .unwrap();

    // TODO: remove hardcoded params
    let socket_address: String = "0.0.0.0:6000".to_string();
    let gas = 300000000000000_u64;
    let deposit = 9 * 10_u128.pow(20);

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
        gas,
        deposit,
        near_server_url,
    )
    .await
    .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: FinalExecutionStatus = client.request("register_node", rpc_params![signed_tx]).await.unwrap();

    println!("response from server: {:?}", response);
}




#[tokio::main]
pub async fn remove_node() {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let near_server_url: String = std::env::var("NEAR_SERVER_URL")
        .expect("NEAR_SERVER_URL must be set.")
        .parse()
        .unwrap();

    // TODO: remove hardcoded params
    let node_id: Number = Number::from(5);
    let gas = 300000000000000_u64;
    let deposit = 0_u128;


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
        gas,
        deposit,
        near_server_url,
    )
    .await
    .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: FinalExecutionStatus = client.request("remove_node", rpc_params![signed_tx]).await.unwrap();

    println!("response from server: {:?}", response);
}



#[tokio::main]
pub async fn set_node_socket_address() {
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let near_server_url: String = std::env::var("NEAR_SERVER_URL")
        .expect("NEAR_SERVER_URL must be set.")
        .parse()
        .unwrap();

    // TODO: remove hardcoded params
    let node_id: Number = Number::from(8);
    let new_socket_address = "127.0.0.1:8000".to_string();
    let gas = 300000000000000_u64;
    let deposit = 0_u128;


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

    let args = json!({ "node_id": node_id.to_string(), "new_socket_address": new_socket_address }).to_string().into_bytes();

    let signed_tx = construct_signed_tx(
        signer_acc_str,
        signer_sk_str,
        contract_id,
        method_name,
        args,
        gas,
        deposit,
        near_server_url,
    )
    .await
    .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: FinalExecutionStatus = client.request("set_node_socket_address", rpc_params![signed_tx]).await.unwrap();

    println!("response from server: {:?}", response);
}






#[tokio::main]
pub async fn get_node_socket_address() {    
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let node_id: Number = Number::from(4);

    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: String = client
        .request("get_node_socket_address", rpc_params![contract_id, node_id.to_string()])
        .await.unwrap();

    println!("response from server: {:?}", response);
}



#[tokio::main]
pub async fn get_node_owner() {    
    let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
        .expect("SEDA_SERVER_URL must be set.")
        .parse()
        .unwrap();
    let node_id: Number = Number::from(4);

    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();

    let client = WsClientBuilder::default().build(&seda_server_url).await.unwrap();
    let response: String = client
        .request("get_node_owner", rpc_params![contract_id, node_id.to_string()])
        .await.unwrap();

    println!("response from server: {:?}", response);
}
