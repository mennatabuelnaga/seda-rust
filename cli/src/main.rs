mod node_commands;
mod helpers;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use node_commands::{register, get_node_socket_address};

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
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
                Box::pin(register());
            }
            Commands::GetNodeSocketAddress => {
                // cargo run --bin seda get-node-socket-address
                Box::pin(get_node_socket_address());
            }
            Commands::Run => seda_node::run(), // cargo run --bin seda run
        }
    } else {
        todo!()
    }

    Ok(())
}
