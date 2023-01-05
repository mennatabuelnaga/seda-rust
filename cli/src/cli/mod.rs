use std::env;

use crate::Result;

mod cli_commands;
use cli_commands::call_cli;

pub struct Cli;

impl Cli {
    #[tokio::main]
    pub async fn handle_cli_commands(mut args: Vec<String>) -> Result<()> {
        // Remove path of binary
        args.remove(0);
        call_cli(&args).await?;
        Ok(())
    }

    pub fn run() -> Result<()> {
        let args: Vec<String> = env::args().collect();

        if let Some(command) = args.get(1) {
            if command == "run" {
                seda_node::run();
            }
        }

        Self::handle_cli_commands(args)
    }
}
