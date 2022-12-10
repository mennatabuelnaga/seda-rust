use clap::{arg, command, Parser, Subcommand};
use seda_config::{overwrite_config_field, CONFIG};

use crate::{errors::CliError, Result};
mod cli_commands;
use cli_commands::*;
#[cfg(feature = "near")]
mod near_backend;

#[cfg(feature = "near")]
pub type CliBackend = near_backend::NearCliBackend;

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
pub struct CliOptions {
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
    RegisterNode {
        #[arg(short, long)]
        socket_address:      String,
        #[arg(long)]
        seda_server_url:     Option<String>,
        #[arg(long)]
        signer_account_id:   Option<String>,
        #[arg(short = 'k', long)]
        secret_key:          Option<String>,
        #[arg(short, long)]
        contract_account_id: Option<String>,
    },
    GetNodes {
        #[arg(short, long)]
        limit:               u64,
        #[arg(short, long, default_value = "0")]
        offset:              u64,
        #[arg(short, long)]
        contract_account_id: Option<String>,
    },
    GetNodeSocketAddress {
        #[arg(short, long)]
        node_id:             u64,
        #[arg(short, long)]
        contract_account_id: Option<String>,
    },
    RemoveNode {
        #[arg(short, long)]
        node_id:             u64,
        #[arg(long)]
        seda_server_url:     Option<String>,
        #[arg(long)]
        signer_account_id:   Option<String>,
        #[arg(short = 'k', long)]
        secret_key:          Option<String>,
        #[arg(short, long)]
        contract_account_id: Option<String>,
    },
    SetNodeSocketAddress {
        #[arg(short, long)]
        node_id:             u64,
        #[arg(short, long)]
        socket_address:      String,
        #[arg(long)]
        seda_server_url:     Option<String>,
        #[arg(long)]
        signer_account_id:   Option<String>,
        #[arg(short = 'k', long)]
        secret_key:          Option<String>,
        #[arg(short, long)]
        contract_account_id: Option<String>,
    },
    GetNodeOwner {
        #[arg(short, long)]
        node_id:             u64,
        #[arg(short, long)]
        contract_account_id: Option<String>,
    },
}

impl CliOptions {
    // This is temporary until we move the execution of these to
    // the runtime.
    #[tokio::main]
    async fn rest_of_options<T: CliCommands>(command: Command) -> Result<()> {
        match command {
            // cargo run --bin seda register-node --socket-address 127.0.0.1:9000
            Command::RegisterNode {
                socket_address,
                seda_server_url,
                signer_account_id,
                secret_key,
                contract_account_id,
            } => {
                {
                    let mut config = CONFIG.blocking_write();
                    overwrite_config_field!(config.seda_server_url, seda_server_url);
                    let node_config = config.node.as_mut().ok_or("Missing config [node] section")?;
                    overwrite_config_field!(node_config.signer_account_id, signer_account_id);
                    overwrite_config_field!(node_config.secret_key, secret_key);
                    overwrite_config_field!(node_config.contract_account_id, contract_account_id);
                }
                T::register_node(socket_address).await?
            }
            // cargo run --bin seda get-nodes --limit 2
            Command::GetNodes {
                limit,
                offset,
                contract_account_id,
            } => {
                {
                    let mut config = CONFIG.blocking_write();
                    let node_config = config.node.as_mut().ok_or("Missing config [node] section")?;
                    overwrite_config_field!(node_config.contract_account_id, contract_account_id);
                }
                T::get_nodes(limit, offset).await?
            }
            // cargo run --bin seda get-node-socket-address --node-id 9
            Command::GetNodeSocketAddress {
                node_id,
                contract_account_id,
            } => {
                {
                    let mut config = CONFIG.blocking_write();
                    let node_config = config.node.as_mut().ok_or("Missing config [node] section")?;
                    overwrite_config_field!(node_config.contract_account_id, contract_account_id);
                }
                T::get_node_socket_address(node_id).await?
            }
            // cargo run --bin seda remove-node --node-id 9
            Command::RemoveNode {
                node_id,
                seda_server_url,
                signer_account_id,
                secret_key,
                contract_account_id,
            } => {
                {
                    let mut config = CONFIG.blocking_write();
                    overwrite_config_field!(config.seda_server_url, seda_server_url);
                    let node_config = config.node.as_mut().ok_or("Missing config [node] section")?;
                    overwrite_config_field!(node_config.signer_account_id, signer_account_id);
                    overwrite_config_field!(node_config.secret_key, secret_key);
                    overwrite_config_field!(node_config.contract_account_id, contract_account_id);
                }
                T::remove_node(node_id).await?
            }
            // cargo run --bin seda set-node-socket-address --node-id 9
            Command::SetNodeSocketAddress {
                node_id,
                socket_address,
                seda_server_url,
                signer_account_id,
                secret_key,
                contract_account_id,
            } => {
                {
                    let mut config = CONFIG.blocking_write();
                    overwrite_config_field!(config.seda_server_url, seda_server_url);
                    let node_config = config.node.as_mut().ok_or("Missing config [node] section")?;
                    overwrite_config_field!(node_config.signer_account_id, signer_account_id);
                    overwrite_config_field!(node_config.secret_key, secret_key);
                    overwrite_config_field!(node_config.contract_account_id, contract_account_id);
                }
                T::set_node_socket_address(node_id, socket_address).await?
            }
            // cargo run --bin seda get-node-owner --node-id 9
            Command::GetNodeOwner {
                node_id,
                contract_account_id,
            } => {
                {
                    let mut config = CONFIG.blocking_write();
                    let node_config = config.node.as_mut().ok_or("Missing config [node] section")?;
                    overwrite_config_field!(node_config.contract_account_id, contract_account_id);
                }
                T::get_node_owner(node_id).await?
            }
            Command::Cli { args } => T::call_cli(&args).await?,
            // The commands `run` are already handled.
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn handle<T: CliCommands>() -> Result<()> {
        let options = CliOptions::parse();
        dotenv::dotenv().ok();

        if let Command::Run { rpc_server_address } = options.command {
            {
                let mut config = CONFIG.blocking_write();
                config
                    .main_chain
                    .as_ref()
                    .ok_or("Missing config [main_chain] section")?;
                let node_config = config.node.as_mut().ok_or("Missing config [node] section")?;
                overwrite_config_field!(node_config.rpc_server_address, rpc_server_address);
            }

            return seda_logger::init(|| {
                seda_node::run::<T::MainChainAdapter>();
                Ok::<_, CliError>(())
            });
        }

        seda_logger::init(|| Self::rest_of_options::<T>(options.command))
    }
}
