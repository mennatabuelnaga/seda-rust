use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams},
    rpc_params,
    ws_client::WsClientBuilder,
};
use near_primitives::views::FinalExecutionStatus;
use seda_adapters::MainChainAdapterTrait;
use serde_json::json;

use crate::{errors::Result, helpers::get_env_var};

const GAS: u64 = 300_000_000_000_000;
const DEPOSIT_FOR_REGISTER_NODE: u128 = 87 * 10_u128.pow(19); // 0.00087 NEAR

async fn view_seda_server(method: &str, params: ArrayParams) -> Result<String> {
    let seda_server_url = get_env_var("SEDA_SERVER_URL")?;

    let client = WsClientBuilder::default().build(&seda_server_url).await?;

    let response = client.request(method, params).await?;

    Ok(response)
}

async fn format_tx_and_request_seda_server<T: MainChainAdapterTrait>(
    method: &str,
    args: Vec<u8>,
    deposit: u128,
) -> Result<FinalExecutionStatus> {
    let seda_server_url = get_env_var("SEDA_SERVER_URL")?;
    let near_server_url = get_env_var("NEAR_SERVER_URL")?;
    let signer_acc_str = get_env_var("SIGNER_ACCOUNT_ID")?;
    let signer_sk_str = get_env_var("SECRET_KEY")?;
    let contract_id = get_env_var("CONTRACT_ACCOUNT_ID")?;

    let signed_tx = T::construct_signed_tx(
        &signer_acc_str,
        &signer_sk_str,
        &contract_id,
        method,
        args,
        GAS,
        deposit,
        &near_server_url,
    )
    .await
    .expect("todo");

    let client = WsClientBuilder::default().build(&seda_server_url).await?;
    let response = client.request(method, rpc_params![signed_tx, near_server_url]).await?;

    Ok(response)
}

#[tokio::main]
pub async fn register_node<T: MainChainAdapterTrait>(socket_address: String) -> Result<()> {
    let method_name = "register_node";

    let response = format_tx_and_request_seda_server::<T>(
        method_name,
        json!({ "socket_address": socket_address }).to_string().into_bytes(),
        DEPOSIT_FOR_REGISTER_NODE,
    )
    .await?;

    println!("response from server: {:?}", response);

    Ok(())
}

#[tokio::main]
pub async fn remove_node<T: MainChainAdapterTrait>(node_id: u64) -> Result<()> {
    let method_name = "remove_node";

    let response = format_tx_and_request_seda_server::<T>(
        method_name,
        json!({ "node_id": node_id.to_string() }).to_string().into_bytes(),
        0_u128,
    )
    .await?;

    println!("response from server: {:?}", response);

    Ok(())
}

#[tokio::main]
pub async fn set_node_socket_address<T: MainChainAdapterTrait>(node_id: u64, new_socket_address: String) -> Result<()> {
    let method_name = "set_node_socket_address";

    let response = format_tx_and_request_seda_server::<T>(
        method_name,
        json!({ "node_id": node_id.to_string(), "new_socket_address": new_socket_address })
            .to_string()
            .into_bytes(),
        0_u128,
    )
    .await?;

    println!("response from server: {:?}", response);

    Ok(())
}

#[tokio::main]
pub async fn get_node_socket_address(node_id: u64) -> Result<()> {
    let near_server_url = get_env_var("NEAR_SERVER_URL")?;
    let contract_id = get_env_var("CONTRACT_ACCOUNT_ID")?;

    let response = view_seda_server(
        "get_node_socket_address",
        rpc_params![contract_id, node_id, near_server_url],
    )
    .await?;

    println!("response from server: {:?}", response);

    Ok(())
}

#[tokio::main]
pub async fn get_nodes(limit: u64, offset: u64) -> Result<()> {
    let near_server_url = get_env_var("NEAR_SERVER_URL")?;
    let contract_id = get_env_var("CONTRACT_ACCOUNT_ID")?;

    let response = view_seda_server("get_nodes", rpc_params![contract_id, limit, offset, near_server_url]).await?;

    println!("response from server: {:?}", response);

    Ok(())
}

#[tokio::main]
pub async fn get_node_owner(node_id: u64) -> Result<()> {
    let near_server_url = get_env_var("NEAR_SERVER_URL")?;
    let contract_id = get_env_var("CONTRACT_ACCOUNT_ID")?;

    let response = view_seda_server("get_node_owner", rpc_params![contract_id, node_id, near_server_url]).await?;

    println!("response from server: {:?}", response);

    Ok(())
}
