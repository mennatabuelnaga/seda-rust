use clap::{arg, command, Parser, Subcommand};
use seda_config::{AppConfig, PartialChainConfigs, PartialLoggerConfig, PartialNearConfig, PartialNodeConfig};
use seda_runtime_sdk::Chain;

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
    #[arg(short, long)]
    chain:           Chain,
    #[command(flatten)]
    pub log_options: PartialLoggerConfig,
    #[command(subcommand)]
    command:         Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        #[command(flatten)]
        node_config:   PartialNodeConfig,
        #[command(flatten)]
        chains_config: PartialChainConfigs,
    },
    Cli {
        args: Vec<String>,
    },
    RegisterNode {
        #[arg(short, long)]
        socket_address: String,
        #[command(flatten)]
        node_config:    PartialNodeConfig,
        #[command(flatten)]
        near_config:    PartialNearConfig,
    },
    GetNodes {
        #[arg(short, long)]
        limit:       u64,
        #[arg(short, long, default_value = "0")]
        offset:      u64,
        #[command(flatten)]
        node_config: PartialNodeConfig,
    },
    GetNodeSocketAddress {
        #[arg(short, long)]
        node_id:     u64,
        #[command(flatten)]
        node_config: PartialNodeConfig,
    },
    RemoveNode {
        #[arg(short, long)]
        node_id:     u64,
        #[command(flatten)]
        node_config: PartialNodeConfig,
        #[command(flatten)]
        near_config: PartialNearConfig,
    },
    SetNodeSocketAddress {
        #[arg(short, long)]
        node_id:        u64,
        #[arg(short, long)]
        socket_address: String,
        #[command(flatten)]
        node_config:    PartialNodeConfig,
        #[command(flatten)]
        near_config:    PartialNearConfig,
    },
    GetNodeOwner {
        #[arg(short, long)]
        node_id:     u64,
        #[command(flatten)]
        node_config: PartialNodeConfig,
    },
    SignTxn {
        #[arg(short, long)]
        signer_account_id: Option<String>,
        #[command(flatten)]
        node_config:       PartialNodeConfig,
    },
}

impl CliOptions {
    // This is temporary until we move the execution of these to
    // the runtime.
    #[tokio::main]
    async fn rest_of_options<T: CliCommands>(config: AppConfig, command: Command) -> Result<()> {
        match command {
            // cargo run cli call mc.mennat0.testnet register_node "{\"socket_address\":\"127.0.0.1:8080\"}"
            // "870000000000000000000"
            Command::RegisterNode {
                socket_address,
                node_config,
                near_config,
            } => {
                let node_config = config.node.to_config(node_config)?;
                let near_config = config.chains.near.to_config(near_config)?;
                T::register_node(
                    &config.seda_server_url,
                    &near_config.chain_rpc_url,
                    &node_config,
                    &socket_address,
                )
                .await?
            }
            // cargo run --bin seda get-nodes --limit 2
            Command::GetNodes {
                limit,
                offset,
                node_config,
            } => {
                let node_config = config.node.to_config(node_config)?;
                T::get_nodes(&config.seda_server_url, &node_config, limit, offset).await?
            }
            // cargo run --bin seda get-node-socket-address --node-id 9
            Command::GetNodeSocketAddress { node_id, node_config } => {
                let node_config = config.node.to_config(node_config)?;
                T::get_node_socket_address(&config.seda_server_url, &node_config, node_id).await?
            }
            // cargo run --bin seda remove-node --node-id 9
            Command::RemoveNode {
                node_id,
                node_config,
                near_config,
            } => {
                let node_config = config.node.to_config(node_config)?;
                let near_config = config.chains.near.to_config(near_config)?;
                T::remove_node(
                    &config.seda_server_url,
                    &near_config.chain_rpc_url,
                    &node_config,
                    node_id,
                )
                .await?
            }
            // cargo run --bin seda set-node-socket-address --node-id 9
            Command::SetNodeSocketAddress {
                node_id,
                socket_address,
                node_config,
                near_config,
            } => {
                let node_config = config.node.to_config(node_config)?;
                let near_config = config.chains.near.to_config(near_config)?;
                T::set_node_socket_address(
                    &config.seda_server_url,
                    &near_config.chain_rpc_url,
                    &node_config,
                    node_id,
                    &socket_address,
                )
                .await?
            }
            // cargo run --bin seda get-node-owner --node-id 9
            Command::GetNodeOwner { node_id, node_config } => {
                let node_config = config.node.to_config(node_config)?;
                T::get_node_owner(&config.seda_server_url, &node_config, node_id).await?
            }
            Command::Cli { args } => T::call_cli(&config.seda_server_url, &args).await?,

            // The commands `run` and `generate-config` are already handled.
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
            seda_node::run(node_config, chains_config);

            return Ok(());
        }

        Self::rest_of_options::<near_backend::NearCliBackend>(config, self.command)
    }
}
