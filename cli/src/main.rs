mod errors;
mod helpers;
mod node_commands;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

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
    Cli { args: Vec<String> },
}

fn main() {
    let options = Options::parse();
    dotenv().ok();

    if let Some(command) = options.command {
        match command {
            Commands::Run => seda_node::run(),
            Commands::Cli { args } => {
                call_cli(args).unwrap();
            }
        }
    }
}
