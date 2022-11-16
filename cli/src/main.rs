mod errors;
mod helpers;
mod node_commands;

use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use node_commands::{
    get_node_owner,
    get_node_socket_address,
    get_nodes,
    register_node,
    remove_node,
    set_node_socket_address,
};

use crate::node_commands::call_cli;

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
    Cli,
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

fn main() {
    let options = Options::parse();
    dotenv().ok();

    if let Some(command) = options.command {
        match command {
            Commands::Run => seda_node::run(),
            Commands::Cli => {
                let args: Vec<String> = env::args().collect();
                call_cli(args).unwrap();
            }

            _ => println!("Nope"),
        }
    } else {
        let args: Vec<String> = env::args().collect();
        println!("test {:?}", args);
        // call_cli(args).unwrap();
    }
}

// fn main() {
//     let options = Options::parse();
//     dotenv().ok();

//     if let Some(command) = options.command {
//         match command {
//             // cargo run --bin seda register-node --socket-address
// 127.0.0.1:9000             Commands::RegisterNode { socket_address } => {
//                 register_node(socket_address).unwrap();
//             }
//             // cargo run --bin seda get-nodes --limit 2
//             Commands::GetNodes { limit, offset } => {
//                 get_nodes(limit, offset).unwrap();
//             }
//             // cargo run --bin seda get-node-socket-address --node-id 9
//             Commands::GetNodeSocketAddress { node_id } => {
//                 get_node_socket_address(node_id).unwrap();
//             }
//             // cargo run --bin seda run
//             Commands::Run => seda_node::run(),
//             // cargo run --bin seda remove-node --node-id 9
//             Commands::RemoveNode { node_id } =>
// remove_node(node_id).unwrap(),             // cargo run --bin seda
// set-node-socket-address --node-id 9 --socket-address 127.0.0.1:9000
//             Commands::SetNodeSocketAddress {
//                 node_id,
//                 socket_address,
//             } => set_node_socket_address(node_id, socket_address).unwrap(),
//             // cargo run --bin seda get-node-owner --node-id 9
//             Commands::GetNodeOwner { node_id } =>
// get_node_owner(node_id).unwrap(),         }
//     } else {
//         todo!()
//     }
// }
