use std::str;

use clap::{Parser, Subcommand};
use seda_runtime_sdk::{
    wasm::{call_self, db_get, db_set, http_fetch, memory_read, memory_write, Promise},
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
<<<<<<< HEAD
    Hello,
    HttpFetch { url: String },
=======
    Run,
<<<<<<< HEAD
>>>>>>> 3433719 (feat(cli): use runtime for cli commands)
=======
    Hello,
>>>>>>> 49cd2da (fix(runtime): Fix issue where http could not be called due missing tokio runtime)
}

fn main() {
    let options = Options::parse();

    if let Some(command) = options.command {
        match command {
<<<<<<< HEAD
<<<<<<< HEAD
            Commands::HttpFetch { url } => {
                http_fetch(&url).start().then(call_self("http_fetch_result", vec![]));
=======
            Commands::Run => {
                http_fetch("https://swapi.dev/api/people/2/")
                    .start()
                    .then(call_self("http_fetch_result", vec![]));
>>>>>>> 49cd2da (fix(runtime): Fix issue where http could not be called due missing tokio runtime)
            }
            Commands::Hello => {
                println!("Hello World from inside wasm");
            }
<<<<<<< HEAD
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
=======
            Commands::Run => println!("DSAasd"),
        }
    }
}
>>>>>>> 3433719 (feat(cli): use runtime for cli commands)
=======
        }
    }
}

#[no_mangle]
fn http_fetch_result() {
    Promise::result(0);
    println!("Result!");
}
>>>>>>> 49cd2da (fix(runtime): Fix issue where http could not be called due missing tokio runtime)
