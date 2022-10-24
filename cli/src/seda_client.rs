use dotenv::dotenv;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use near_primitives::transaction::SignedTransaction;
use serde_json::Number;

pub async fn register_node(signed_tx: SignedTransaction, server_url: String) -> anyhow::Result<()> {
    let client = WsClientBuilder::default().build(&server_url).await?;

    let response: String = client.request("register_node", rpc_params![signed_tx]).await?;

    println!("response from server: {:?}", response);

    Ok(())
}

pub async fn get_node_socket_address(server_url: String, node_id: Number) -> anyhow::Result<()> {
    dotenv().ok();

    let contract_id: String = std::env::var("CONTRACT_ACCOUNT_ID")
        .expect("CONTRACT_ACCOUNT_ID must be set.")
        .parse()
        .unwrap();

    let client = WsClientBuilder::default().build(&server_url).await?;
    let response: String = client
        .request("get_node_socket_address", rpc_params![contract_id, node_id.to_string()])
        .await?;

    println!("response from server: {:?}", response);

    Ok(())
}
