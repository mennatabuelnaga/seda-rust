use clap::{command, Parser, Subcommand};
use seda_config::{AppConfig, PartialLoggerConfig};

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
    Node {
        #[command(subcommand)]
        node: Node,
    },
    SubChain {
        #[command(subcommand)]
        sub_chain: SubChain,
    },
}

impl Command {
    #[tokio::main]
    pub async fn handle(self, config: AppConfig) -> Result<()> {
        match self {
            Self::Node { node } => node.handle(config),
            Self::SubChain { sub_chain } => sub_chain.handle(),
        }
    }
}
