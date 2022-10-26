mod helpers;
mod node_commands;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use node_commands::{get_node_owner, get_node_socket_address, register_node, remove_node, set_node_socket_address};

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
    Run,
    RegisterNode {
        #[arg(short, long)]
        socket_address: String,
    },
    GetNodeSocketAddress {
        #[arg(short, long)]
        node_id: u64,
    },
    RemoveNode {
        #[arg(short, long)]
        node_id: u64,
    },
    SetNodeSocketAddress {
        #[arg(short, long)]
        node_id:        u64,
        #[arg(short, long)]
        socket_address: String,
    },
    GetNodeOwner {
        #[arg(short, long)]
        node_id: u64,
    },
}

fn main() {
    let options = Options::parse();
    dotenv().ok();

    if let Some(command) = options.command {
        match command {
            Commands::RegisterNode { socket_address } => {
                // cargo run --bin seda register-node --socket-address 127.0.0.1:9000
                register_node(socket_address)
            }
            Commands::GetNodeSocketAddress { node_id } => {
                // cargo run --bin seda get-node-socket-address --node-id 9
                get_node_socket_address(node_id);
            }
            Commands::Run => seda_node::run(), // cargo run --bin seda run
            Commands::RemoveNode { node_id } => remove_node(node_id), // cargo run --bin seda remove-node --node-id 9
            Commands::SetNodeSocketAddress {
                node_id,
                socket_address,
            } => set_node_socket_address(node_id, socket_address), // cargo run --bin seda set-node-socket-address
            Commands::GetNodeOwner { node_id } => get_node_owner(node_id), // cargo run --bin seda get-node-owner
        }
    } else {
        todo!()
    }
}
