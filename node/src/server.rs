use std::str::FromStr;

use jsonrpsee::server::ServerBuilder;
use jsonrpsee::RpcModule;
use seda_contracts_adapter::mc_client::{call_change_method, call_view_method};
use serde_json::{json, Number};
use tracing_subscriber::util::SubscriberInitExt;

pub async fn run() -> anyhow::Result<()> {
    println!("Starting server...");

    start_server().await?;

    // How do we keep the server running without using an infinite loop?
    loop {}
}

pub async fn start_server() -> anyhow::Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
        .add_directive("jsonrpsee[method_call{name = \"get_node_socket_address\"}]=trace".parse()?)
        .add_directive("jsonrpsee[method_call{name = \"register_node\"}]=trace".parse()?);

    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish()
        .try_init()?;

    let server = ServerBuilder::default().build("127.0.0.1:12345").await?;
    let mut module = RpcModule::new(());

    // register methods
    module
        .register_async_method("get_node_socket_address", |params, _| async move {
            let method_name = "get_node_socket_address".to_string();

            let received_params: Vec<String> = params.parse().unwrap();

            // let node_id: Number = params.one()?;
            let contract_id = received_params[0].to_string();
            let node_id = Number::from_str((received_params[1]).as_str()).unwrap();

            let args = json!({"node_id": node_id.to_string()}).to_string().into_bytes();
            let server_addr = "https://rpc.testnet.near.org".to_string();

            let status = call_view_method(contract_id, method_name, args, server_addr).await;
            Ok(status)
        })
        .unwrap();

    module
        .register_async_method("register_node", |params, _| async move {
            // let received_params: Vec<String> = params.parse().unwrap();
            let signed_tx = params.one()?;
            let server_addr = "https://rpc.testnet.near.org".to_string();

            call_change_method(signed_tx, server_addr).await.unwrap();
            Ok("Successful register_node")
        })
        .unwrap();

    let addr = server.local_addr()?;
    let handle = server.start(module)?;
    tokio::spawn(handle.stopped());

    let url = format!("ws://{}", addr);
    println!("server started on url: {:?}", url);
    Ok(())
}
