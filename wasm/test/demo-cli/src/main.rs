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
        contract_id: String, method_name: String, args: String
    },
    Change {
        contract_id: String, method_name: String, args: String
    },
}

fn main() {
    let options = Options::parse();

    if let Some(command) = options.command {
        match command {
            // cargo run cli http-fetch "https://www.breakingbadapi.com/api/characters/1"
            Commands::HttpFetch { url } => {
                http_fetch(&url).start().then(call_self("http_fetch_result", vec![]));
            }
            Commands::Hello => {
                println!("Hello World from inside wasm");
            },
            //cargo run cli view mc.mennat0.testnet get_node_owner "{\"node_id\":\"12\"}" "https://rpc.testnet.near.org"
            Commands::View{contract_id, method_name, args} => {
                chain_view(contract_id, method_name, args.into_bytes()).start()
                .then(call_self("chain_view_test_success", vec![]));
            },
            // register_node serialized signed txn
            // cargo run cli change "[15, 0, 0, 0, 109, 101, 110, 110, 97, 116, 48, 46, 116, 101, 115, 116, 110, 101, 116, 0, 207, 192, 197, 140, 145, 245, 110, 118, 149, 219, 145, 0, 54, 55, 137, 187, 158, 138, 61, 188, 152, 43, 17, 195, 204, 187, 85, 107, 135, 185, 210, 83, 159, 122, 37, 71, 54, 91, 0, 0, 18, 0, 0, 0, 109, 99, 46, 109, 101, 110, 110, 97, 116, 48, 46, 116, 101, 115, 116, 110, 101, 116, 42, 186, 39, 43, 77, 157, 173, 70, 179, 6, 6, 12, 253, 88, 118, 29, 206, 214, 167, 49, 180, 140, 70, 49, 63, 162, 233, 193, 80, 83, 130, 196, 1, 0, 0, 0, 2, 13, 0, 0, 0, 114, 101, 103, 105, 115, 116, 101, 114, 95, 110, 111, 100, 101, 35, 0, 0, 0, 123, 34, 115, 111, 99, 107, 101, 116, 95, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 56, 48, 34, 125, 0, 192, 110, 49, 217, 16, 1, 0, 0, 0, 216, 221, 138, 230, 172, 41, 47, 0, 0, 0, 0, 0, 0, 0, 0, 217, 172, 125, 56, 237, 9, 130, 7, 102, 136, 247, 153, 82, 7, 166, 206, 180, 105, 145, 151, 51, 182, 189, 251, 149, 166, 126, 87, 78, 149, 239, 10, 130, 42, 221, 226, 165, 32, 228, 57, 88, 125, 98, 135, 118, 167, 76, 73, 91, 233, 35, 96, 43, 169, 143, 240, 10, 164, 31, 29, 111, 246, 3, 14]" "https://rpc.testnet.near.org"
            Commands::Change{contract_id, method_name, args} => {
                chain_change(contract_id, method_name, args.into_bytes()).start()
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
