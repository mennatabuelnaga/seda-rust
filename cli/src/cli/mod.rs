use clap::{command, Parser, Subcommand};
use seda_config::{AppConfig, PartialChainConfigs, PartialLoggerConfig};

use crate::Result;

mod commands;
use commands::*;

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
pub struct CliOptions {
    #[command(flatten)]
    pub log_options: PartialLoggerConfig,
    #[command(subcommand)]
    pub command:     Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Node(Node),
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
            Self::Node(node_command) => node_command.handle(config),
            Self::SubChain {
                chains_config,
                sub_chain_command,
            } => sub_chain_command.handle(config, chains_config),
        }
    }
}
