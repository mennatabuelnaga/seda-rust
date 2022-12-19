use clap::{arg, command, Parser, Subcommand};
use seda_chain_adapters::{AnotherMainChain, NearMainChain};
use seda_config::CONFIG;
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
    SignTxn {
        #[arg(short, long)]
        signer_account_id:   Option<String>,
        #[arg(short = 'k', long)]
        secret_key:          Option<String>,
        #[arg(short, long)]
        contract_account_id: Option<String>,
        #[arg(short, long)]
        method_name:         String,
        #[arg(short, long)]
        args:                String,
        #[arg(short, long)]
        gas:                 u64,
        #[arg(short, long)]
        deposit:             u128,
    },
}

impl CliOptions {
    // This is temporary until we move the execution of these to
    // the runtime.
    #[tokio::main]
    async fn rest_of_options<T: CliCommands>(command: Command) -> Result<()> {
        match command {
            // cargo run cli call mc.mennat0.testnet register_node "{\"socket_address\":\"127.0.0.1:8080\"}"
            // "870000000000000000000"
            Command::RegisterNode {
                socket_address,
                seda_server_url,
                signer_account_id,
                secret_key,
                contract_account_id,
            } => {
                {
                    let mut config = CONFIG.blocking_write();
                    if let Some(seda_server_url) = seda_server_url {
                        config.seda_server_url = seda_server_url;
                    }

                    if let Some(signer_account_id) = signer_account_id {
                        config.node.signer_account_id = signer_account_id;
                    }
                    if let Some(secret_key) = secret_key {
                        config.node.secret_key = secret_key;
                    }
                    if let Some(contract_account_id) = contract_account_id {
                        config.node.contract_account_id = contract_account_id;
                    }
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
                    if let Some(contract_account_id) = contract_account_id {
                        config.node.contract_account_id = contract_account_id;
                    }
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
                    if let Some(contract_account_id) = contract_account_id {
                        config.node.contract_account_id = contract_account_id;
                    }
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
                    if let Some(seda_server_url) = seda_server_url {
                        config.seda_server_url = seda_server_url;
                    }

                    if let Some(signer_account_id) = signer_account_id {
                        config.node.signer_account_id = signer_account_id;
                    }
                    if let Some(secret_key) = secret_key {
                        config.node.secret_key = secret_key;
                    }
                    if let Some(contract_account_id) = contract_account_id {
                        config.node.contract_account_id = contract_account_id;
                    }
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
                    if let Some(seda_server_url) = seda_server_url {
                        config.seda_server_url = seda_server_url;
                    }

                    if let Some(signer_account_id) = signer_account_id {
                        config.node.signer_account_id = signer_account_id;
                    }
                    if let Some(secret_key) = secret_key {
                        config.node.secret_key = secret_key;
                    }
                    if let Some(contract_account_id) = contract_account_id {
                        config.node.contract_account_id = contract_account_id;
                    }
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
                    if let Some(contract_account_id) = contract_account_id {
                        config.node.contract_account_id = contract_account_id;
                    }
                }
                T::get_node_owner(node_id).await?
            }
            Command::Cli { args } => T::call_cli(&args).await?,

            // The commands `run` and `generate-config` are already handled.
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn handle() -> Result<()> {
        let options = CliOptions::parse();

        if let Command::Run { rpc_server_address } = options.command {
            {
                let mut config = CONFIG.blocking_write();

                if let Some(rpc_server_address) = rpc_server_address {
                    config.node.rpc_server_address = rpc_server_address;
                }
            }

            match options.chain {
                Chain::Another => seda_node::run::<AnotherMainChain>(),
                Chain::Near => seda_node::run::<NearMainChain>(),
            }

            return Ok(());
        }

        match options.chain {
            Chain::Another => unimplemented!(),
            Chain::Near => Self::rest_of_options::<near_backend::NearCliBackend>(options.command),
        }
    }
}
