use clap::{arg, command, Parser, Subcommand};
use seda_adapters::MainChainAdapterTrait;

use crate::Result;
mod node_commands;

// TODO eventually break these up into files.
// makes cli structure cleaner to work with.
use node_commands::{
    get_node_owner,
    get_node_socket_address,
    get_nodes,
    register_node,
    remove_node,
    set_node_socket_address,
};

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
pub struct Options {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        // TODO Temporary should be moved to config file?
        server_address: String,
    },
    RegisterNode {
        #[arg(short, long)]
        socket_address: String,
    },
    GetNodes {
        #[arg(short, long)]
        limit:  u64,
        #[arg(short, long, default_value = "0")]
        offset: u64,
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

impl Options {
    pub async fn handle<T: MainChainAdapterTrait>() -> Result<()> {
        let options = Options::parse();
        dotenv::dotenv().ok();

        match options.command {
            // cargo run --bin seda register-node --socket-address 127.0.0.1:9000
            Some(Commands::RegisterNode { socket_address }) => {
                register_node::<T>(socket_address).await?;
            }
            // cargo run --bin seda get-nodes --limit 2
            Some(Commands::GetNodes { limit, offset }) => {
                get_nodes(limit, offset).await?;
            }
            // cargo run --bin seda get-node-socket-address --node-id 9
            Some(Commands::GetNodeSocketAddress { node_id }) => {
                get_node_socket_address(node_id).await?;
            }
            // cargo run --bin seda run
            Some(Commands::Run { server_address }) => seda_node::run::<T>(&server_address),
            // cargo run --bin seda remove-node --node-id 9
            Some(Commands::RemoveNode { node_id }) => remove_node::<T>(node_id).await?,
            // cargo run --bin seda set-node-socket-address --node-id 9 --socket-address 127.0.0.1:9000
            Some(Commands::SetNodeSocketAddress {
                node_id,
                socket_address,
            }) => set_node_socket_address::<T>(node_id, socket_address).await?,
            // cargo run --bin seda get-node-owner --node-id 9
            Some(Commands::GetNodeOwner { node_id }) => get_node_owner(node_id).await?,
            None => todo!(),
        }

        Ok(())
    }
}
