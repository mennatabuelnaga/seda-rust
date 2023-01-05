use std::env;

use clap::{Parser, Subcommand};
use seda_runtime_sdk::{
    wasm::{call_self, chain_call, chain_view, db_set, http_fetch, log, Promise},
    Chain,
    PromiseStatus,
};

mod commands;
use commands::{get_node, get_nodes, register_node, unregister_node, update_node, UpdateNode};

#[derive(Debug, Parser)]
#[clap(bin_name = "seda")]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
struct Options {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run,
    Hello,
    HttpFetch {
        url: String,
    },
    View {
        chain:       Chain,
        contract_id: String,
        method_name: String,
        args:        String,
    },
    Call {
        chain:       Chain,
        contract_id: String,
        method_name: String,
        args:        String,
        deposit:     String,
    },
    GetNodes {
        offset: u64,
        limit:  u64,
    },
    GetNode {
        node_id: u64,
    },
    RegisterNode {
        socket_address: String,
        deposit:        String,
    },
    UpdateNode {
        node_id: u64,
        command: UpdateNode,
    },
    UnregisterNode {
        node_id: u64,
    },
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = Options::parse_from(args);
    log!(seda_runtime_sdk::Level::Debug, "options: {options:?}");
    println!("Hello Wasm CLI!");

    if let Some(command) = options.command {
        match command {
            // cargo run -- -c near cli http-fetch "https://www.breakingbadapi.com/api/characters/1"
            Commands::HttpFetch { url } => {
                http_fetch(&url).start().then(call_self("http_fetch_result", vec![]));
            }
            Commands::Hello => {
                println!("Hello World from inside wasm");
            }
            // TODO how to remove double near specification here?
            // cargo run -- -c near cli view near mc.mennat0.testnet get_nodes "{\"offset\":\"0\",\"limit\":\"2\"}"
            Commands::View {
                chain,
                contract_id,
                method_name,
                args,
            } => {
                chain_view(chain, contract_id, method_name, args.into_bytes())
                    .start()
                    .then(call_self("chain_view_test_success", vec![]));
            }
            // cargo run -- -c near cli call near mc.mennat0.testnet register_node
            // "{\"socket_address\":\"127.0.0.1:8080\"}" "870000000000000000000"
            Commands::Call {
                chain,
                contract_id,
                method_name,
                args,
                deposit,
            } => {
                chain_call(chain, contract_id, method_name, args.into_bytes(), deposit)
                    .start()
                    .then(call_self("chain_call_test_success", vec![]));
            }
            // cargo run -- -c near cli get-nodes 0 2
            Commands::GetNodes { offset, limit } => {
                get_nodes(limit, offset);
            }
            // cargo run -- -c near cli get-node 1
            Commands::GetNode { node_id } => {
                get_node(node_id);
            }
            // cargo run -- -c near cli register-node 127.0.0.1:8080 870000000000000000000
            Commands::RegisterNode {
                socket_address,
                deposit,
            } => {
                register_node(socket_address, deposit);
            }
            // cargo run -- -c near cli update-node 16 "SetSocketAddress(127.0.0.1:8000)"
            Commands::UpdateNode { node_id, command } => {
                update_node(node_id, command);
            }
            // cargo run -- -c near cli unregister-node 1
            Commands::UnregisterNode { node_id } => {
                unregister_node(node_id);
            }
            Commands::Run => {
                // This command is only handled by the actual node, not by the WASM
                unreachable!();
            }
        }
    }
}

#[no_mangle]
fn http_fetch_result() {
    let result = Promise::result(0);

    let value_to_store: String = match result {
        PromiseStatus::Fulfilled(vec) => String::from_utf8(vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };

    println!("Value: {value_to_store}");
}

#[no_mangle]
fn chain_view_test_success() {
    let result = Promise::result(0);
    let value_to_store: String = match result {
        PromiseStatus::Fulfilled(vec) => String::from_utf8(vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };
    println!("Value: {value_to_store}");

    db_set("chain_view_result", &value_to_store).start();
}

#[no_mangle]
fn chain_call_test_success() {
    let result = Promise::result(0);
    let value_to_store: String = match result {
        PromiseStatus::Fulfilled(vec) => String::from_utf8(vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };
    println!("Value: {value_to_store}");
    db_set("chain_call_result", &value_to_store).start();
}
