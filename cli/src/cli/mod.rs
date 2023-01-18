use clap::{command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use seda_config::{AppConfig, PartialChainConfigs, PartialLoggerConfig};

use crate::Result;

mod commands;
use commands::*;

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version)]
#[command(propagate_version = true)]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
#[command(next_line_help = true)]
pub struct CliOptions {
    #[command(flatten)]
    pub log_options: PartialLoggerConfig,
    #[command(subcommand)]
    pub command:     Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    // seda generate bash
    /// Generates an auto-completion file content for the specified shell.
    Generate {
        /// The shell to generate the auto-completion for.
        shell: Shell,
    },
    // seda node run
    /// Runs the SEDA node.
    Run(Run),
    /// Commands to interact with the SEDA node.
    Node {
        #[command(flatten)]
        chains_config:    PartialChainConfigs,
        #[command(subcommand)]
        sub_node_command: Node,
    },
    #[cfg(debug_assertions)]
    /// Debug commands to help interact with sub-chains.
    SubChain {
        #[command(flatten)]
        chains_config:     PartialChainConfigs,
        #[command(subcommand)]
        sub_chain_command: SubChain,
    },
}

impl Command {
    pub fn handle(self, config: AppConfig) -> Result<()> {
        match self {
            Self::Generate { shell } => {
                let mut cmd = CliOptions::command();
                let cmd_name = cmd.get_name().to_string();
                generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
                Ok(())
            }
            Self::Node {
                chains_config,
                sub_node_command,
            } => sub_node_command.handle(config, chains_config),
            Self::Run(run_command) => run_command.handle(config),
            #[cfg(debug_assertions)]
            Self::SubChain {
                chains_config,
                sub_chain_command,
            } => sub_chain_command.handle(config, chains_config),
        }
    }
}
