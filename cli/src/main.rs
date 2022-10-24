use clap::{Parser, Subcommand};
use dotenv::dotenv;
use helpers::construct_signed_tx;
use serde_json::{json, Number};

pub mod helpers;
pub mod seda_client;
#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the seda protocol.", long_about = None)]
struct Options {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Sends a JsonRPC message to the node's server.
    Register,
    /// Runs the SEDA node
    Run,

    GetNodeSocketAddress,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    dotenv().ok();

    if let Some(command) = options.command {
        match command {
            Commands::Register => {
                // cargo run --bin seda register
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

                let signed_tx_request = construct_signed_tx(
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
                seda_client::register_node(signed_tx_request, seda_server_url).await?;
            }
            Commands::GetNodeSocketAddress => {
                // cargo run --bin seda get-node-socket-address
                let seda_server_url: String = std::env::var("SEDA_SERVER_URL")
                    .expect("SEDA_SERVER_URL must be set.")
                    .parse()
                    .unwrap();
                let node_id: Number = Number::from(4);
                seda_client::get_node_socket_address(seda_server_url, node_id).await?;
            }
            Commands::Run => seda_node::run(), // cargo run --bin seda run
        }
    } else {
        todo!()
    }

    Ok(())
}
