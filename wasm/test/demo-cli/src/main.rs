use std::str;

use clap::{Parser, Subcommand};

use seda_runtime_sdk::{
    wasm::{call_self, http_fetch, Promise},
    PromiseStatus,
};


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
    Hello,
    HttpFetch { url: String },
    View {
        contract_id: String, method_name: String, args: String, server_addr: String
    },
    Change {
        signed_tx: String, server_addr: String
    },
}

fn main() {
    let options = Options::parse();

    if let Some(command) = options.command {
        match command {
            Commands::HttpFetch { url } => {
                http_fetch(&url).start().then(call_self("http_fetch_result", vec![]));
            }
            Commands::Hello => {
                println!("Hello World from inside wasm");
            },
            Commands::View{contract_id, method_name, args, server_addr} => {
                chain_interactor_view(contract_id, method_name, args.into_bytes(), server_addr).start()
                .then(call_self("chain_view_test_success", vec![]));
            },

            Commands::Change{signed_tx, server_addr} => {
                chain_interactor_change(signed_tx.into_bytes(), server_addr).start()
                .then(call_self("chain_change_test_success", vec![]));
            },
        }
    }
}

#[no_mangle]
fn http_fetch_result() {
    let result = Promise::result(0);

    let value_to_print: String = match result {
        PromiseStatus::Fulfilled(vec) => String::from_utf8(vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };

    println!("Value: {value_to_print}");
}



#[no_mangle]
fn chain_view_test_success() {
    let result = Promise::result(0);
    // let value_to_store = String::from_utf8(result).unwrap();
    let value_to_print: String = match result {
        PromiseStatus::Fulfilled(vec) => String::from_utf8(vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };
    println!("Value: {value_to_print}");

    db_set("chain_view_result", &value_to_print).start();
}



#[no_mangle]
fn chain_change_test_success() {
    let result = Promise::result(0);
    // let value_to_store = String::from_utf8(result).unwrap();
    let value_to_print: String = match result {
        PromiseStatus::Fulfilled(vec) => String::from_utf8(vec).unwrap(),
        _ => "Promise failed..".to_string(),
    };
    println!("Value: {value_to_print}");
    db_set("chain_change_result", &value_to_print).start();
}
