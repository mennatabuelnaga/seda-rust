use clap::{arg, command, Parser, Subcommand};
use seda_config::overwrite_config_field;

use crate::{config::AppConfig, Result};
mod cli_commands;
use cli_commands::*;
mod near_backend;
pub use near_backend::NearCliBackend;

#[derive(Parser)]
#[command(name = "seda")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.1.0")]
#[command(about = "For interacting with the SEDA protocol.", long_about = None)]
pub struct CliOptions {
    #[arg(short, long)]
    config_file: Option<std::path::PathBuf>,
    #[command(subcommand)]
    command:     Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    GenerateConfig,
    Run {
        #[arg(short, long)]
        seda_server_url: Option<String>,
    },
    RegisterNode {
        #[arg(short, long)]
        socket_address:      String,
        #[arg(short, long)]
        seda_server_url:     Option<String>,
        #[arg(short, long)]
        signer_account_id:   Option<String>,
        #[arg(short, long)]
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
        #[arg(short, long)]
        seda_server_url:     Option<String>,
        #[arg(short, long)]
        signer_account_id:   Option<String>,
        #[arg(short, long)]
        secret_key:          Option<String>,
        #[arg(short, long)]
        contract_account_id: Option<String>,
    },
    SetNodeSocketAddress {
        #[arg(short, long)]
        node_id:             u64,
        #[arg(short, long)]
        socket_address:      String,
        #[arg(short, long)]
        seda_server_url:     Option<String>,
        #[arg(short, long)]
        signer_account_id:   Option<String>,
        #[arg(short, long)]
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
    async fn rest_of_options<T: CliCommands>(
        mut config: AppConfig<T::MainChainAdapter>,
        command: Command,
    ) -> Result<()> {
        match command {
            // cargo run --bin seda register-node --socket-address 127.0.0.1:9000
            Command::RegisterNode {
                socket_address,
                seda_server_url,
                signer_account_id,
                secret_key,
                contract_account_id,
            } => {
                overwrite_config_field!(config.seda_server_url, seda_server_url);
                overwrite_config_field!(config.signer_account_id, signer_account_id);
                overwrite_config_field!(config.secret_key, secret_key);
                overwrite_config_field!(config.contract_account_id, contract_account_id);
                T::register_node(&config, socket_address).await?
            }
            // cargo run --bin seda get-nodes --limit 2
            Command::GetNodes {
                limit,
                offset,
                contract_account_id,
            } => {
                overwrite_config_field!(config.contract_account_id, contract_account_id);
                T::get_nodes(&config, limit, offset).await?
            }
            // cargo run --bin seda get-node-socket-address --node-id 9
            Command::GetNodeSocketAddress {
                node_id,
                contract_account_id,
            } => {
                overwrite_config_field!(config.contract_account_id, contract_account_id);
                T::get_node_socket_address(&config, node_id).await?
            }
            // cargo run --bin seda remove-node --node-id 9
            Command::RemoveNode {
                node_id,
                seda_server_url,
                signer_account_id,
                secret_key,
                contract_account_id,
            } => {
                overwrite_config_field!(config.seda_server_url, seda_server_url);
                overwrite_config_field!(config.signer_account_id, signer_account_id);
                overwrite_config_field!(config.secret_key, secret_key);
                overwrite_config_field!(config.contract_account_id, contract_account_id);
                T::remove_node(&config, node_id).await?
            }
            // cargo run --bin seda set-node-socket-address --node-id 9 --socket-address 127.0.0.1:9000
            Command::SetNodeSocketAddress {
                node_id,
                socket_address,
                seda_server_url,
                signer_account_id,
                secret_key,
                contract_account_id,
            } => {
                overwrite_config_field!(config.seda_server_url, seda_server_url);
                overwrite_config_field!(config.signer_account_id, signer_account_id);
                overwrite_config_field!(config.secret_key, secret_key);
                overwrite_config_field!(config.contract_account_id, contract_account_id);
                T::set_node_socket_address(&config, node_id, socket_address).await?
            }
            // cargo run --bin seda get-node-owner --node-id 9
            Command::GetNodeOwner {
                node_id,
                contract_account_id,
            } => {
                overwrite_config_field!(config.contract_account_id, contract_account_id);
                T::get_node_owner(&config, node_id).await?
            }
            // The commands `run` and `generate-config` are already handled.
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn handle<T: CliCommands>() -> Result<()> {
        let options = CliOptions::parse();
        dotenv::dotenv().ok();

        match options.command {
            Command::GenerateConfig => {
                return AppConfig::<T::MainChainAdapter>::create_template_from_path("./template_config.toml");
            }
            Command::Run { seda_server_url } => {
                let mut config = AppConfig::<T::MainChainAdapter>::read_from_optional_path(options.config_file)?;
                overwrite_config_field!(config.seda_server_url, seda_server_url);
                seda_node::run::<T::MainChainAdapter>(
                    &config.node_config.ok_or("Config [node_config] section")?,
                    &config
                        .seda_server_url
                        .ok_or("seda_server_url from cli, env var or config file.")?,
                );
                return Ok(());
            }
            _ => {}
        }

        let config = AppConfig::<T::MainChainAdapter>::read_from_optional_path(options.config_file)?;
        Self::rest_of_options::<T>(config, options.command)
    }
}
