use clap::{arg, command, Parser, Subcommand};
use seda_config::CONFIG;
use seda_runtime_sdk::Chain;

use crate::Result;

mod cli_commands;
use cli_commands::call_cli;

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
pub struct CliOptions {
    #[arg(short, long)]
    chain:   Chain,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        #[arg(short, long)]
        rpc_server_address: Option<String>,
    },
    Cli {
        args: Vec<String>,
    },
}

impl CliOptions {
    // This is temporary until we move the execution of these to
    // the runtime.
    #[tokio::main]
    async fn rest_of_options(command: Command) -> Result<()> {
        match command {
            Command::Cli { args } => call_cli(&args).await?,

            // The commands `run` and `generate-config` are already handled.
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn handle() -> Result<()> {
        let options = CliOptions::parse();

        // cargo run -- -c near run
        if let Command::Run { rpc_server_address } = options.command {
            {
                let mut config = CONFIG.blocking_write();
                if let Some(rpc_server_address) = rpc_server_address {
                    config.node.rpc_server_address = rpc_server_address;
                }
            }
            seda_node::run();

            return Ok(());
        }

        Self::rest_of_options(options.command)
    }
}
