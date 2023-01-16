use clap::{arg, command, Parser, Subcommand};
use seda_config::{
    AppConfig,
    PartialChainConfigs,
    PartialDepositAndContractID,
    PartialLoggerConfig,
    PartialNodeConfig,
};

// use seda_runtime_sdk::Chain;
use crate::Result;

mod cli_commands;
use cli_commands::*;

mod near_backend;

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
pub struct CliOptions {
    #[command(flatten)]
    pub log_options: PartialLoggerConfig,
    #[command(subcommand)]
    command:         Command,
}

/// Update node commands
#[derive(Clone, Debug, Subcommand)]
pub enum UpdateNode {
    AcceptOwnership,
    SetPendingOwner {
        #[arg(short, long)]
        owner: String,
    },
    SetSocketAddress {
        #[arg(short, long)]
        address: String,
    },
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        #[command(flatten)]
        node_config:   PartialNodeConfig,
        #[command(flatten)]
        chains_config: PartialChainConfigs,
    },
    GetNodes {
        #[arg(short, long)]
        offset:  u64,
        #[arg(short, long)]
        limit:   u64,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
    GetNode {
        #[arg(short, long)]
        node_id: u64,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
    RegisterNode {
        #[arg(short, long)]
        socket_address: String,
        #[arg(short, long)]
        deposit:        String,
        #[command(flatten)]
        details:        PartialDepositAndContractID,
    },
    UpdateNode {
        #[arg(short, long)]
        node_id: u64,
        #[command(subcommand)]
        command: UpdateNode,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
    UnregisterNode {
        #[arg(short, long)]
        node_id: u64,
        #[command(flatten)]
        details: PartialDepositAndContractID,
    },
}

impl CliOptions {
    // This is temporary until we move the execution of these to
    // the runtime.
    #[tokio::main]
    async fn rest_of_options<T: CliCommands>(config: AppConfig, command: Command) -> Result<()> {
        match command {
            // make run -- get-nodes --limit 2
            Command::GetNodes { limit, offset, details } => {
                let details = config.node.to_deposit_and_contract_id(details)?;
                T::get_nodes(&config.seda_server_url, details, limit, offset).await?
            }
            // The run command is already handled.
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn handle(self, config: AppConfig) -> Result<()> {
        if let Command::Run {
            node_config,
            chains_config,
        } = self.command
        {
            let node_config = config.node.to_config(node_config)?;
            let chains_config = config.chains.to_config(chains_config)?;
            seda_node::run(&config.seda_server_url, node_config, chains_config);

            return Ok(());
        }

        Self::rest_of_options::<near_backend::NearCliBackend>(config, self.command)
    }
}
