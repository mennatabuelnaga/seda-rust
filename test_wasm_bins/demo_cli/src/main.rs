use clap::{Parser, Subcommand};
use seda_runtime_sdk::wasm::{call_self, db_get, db_set, http_fetch, memory_read, memory_write, Promise};

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
    Hello,
}

fn main() {
    let options = Options::parse();

    if let Some(command) = options.command {
        match command {
            Commands::Run => {
                http_fetch("https://swapi.dev/api/people/2/")
                    .start()
                    .then(call_self("http_fetch_result", vec![]));
            }
            Commands::Hello => {
                println!("Hello World from inside wasm");
            }
        }
    }
}

#[no_mangle]
fn http_fetch_result() {
    Promise::result(0);
    println!("Result!");
}
