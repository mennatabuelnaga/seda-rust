mod node_commands;
mod helpers;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use node_commands::{register, get_node_socket_address, remove_node, set_node_socket_address, get_node_owner};

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

    Register,
    GetNodeSocketAddress,
    RemoveNode,
    SetNodeSocketAddress,
    GetNodeOwner,


}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    dotenv().ok();

    if let Some(command) = options.command {
        match command {
            Commands::Register => {
                // cargo run --bin seda register
                register()
            }
            Commands::GetNodeSocketAddress => {
                // cargo run --bin seda get-node-socket-address
                get_node_socket_address();
            }
            Commands::Run => seda_node::run(), // cargo run --bin seda run
            Commands::RemoveNode => remove_node(),// cargo run --bin seda remove-node
            Commands::SetNodeSocketAddress => set_node_socket_address(),// cargo run --bin seda set-node-socket-address
            Commands::GetNodeOwner => get_node_owner(), // cargo run --bin seda get-node-owner
        }
    } else {
        todo!()
    }

    Ok(())
}
