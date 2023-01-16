use std::env;

use crate::Result;

mod cli_commands;
use clap::Command;
use cli_commands::call_cli;

pub struct Cli;

impl Cli {
    #[tokio::main]
    pub async fn handle_cli_commands() -> Result<()> {
        let mut args: Vec<String> = env::args().collect();

        // Remove path of binary for use with runtime
        args.remove(0);
        call_cli(&args).await?;

        Ok(())
    }

    pub fn run() -> Result<()> {
        let run_subcommand = Command::new("run");
        let mut cmd = Command::new("seda").ignore_errors(true).subcommand(run_subcommand);
        cmd.build();

        let matches = cmd.get_matches();
        if matches.subcommand_matches("run").is_some() {
            seda_node::run()
        }

        Self::handle_cli_commands()
    }
}
