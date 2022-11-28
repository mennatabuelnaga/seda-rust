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
            }
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
