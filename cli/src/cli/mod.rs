use clap::{arg, command, Parser, Subcommand};

use crate::{config::Config, Result};
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
		// Todo consider moving this only to relevant commands.
    #[arg(short, long)]
    config_file: Option<std::path::PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    GenerateConfig,
    Run {
        #[arg(short, long)]
        server_address: String,
    },
    RegisterNode {
        #[arg(short, long)]
        socket_address: String,
    },
    GetNodes {
        #[arg(short, long)]
        limit: u64,
        #[arg(short, long, default_value = "0")]
        offset: u64,
    },
    GetNodeSocketAddress {
        #[arg(short, long)]
        node_id: u64,
    },
    RemoveNode {
        #[arg(short, long)]
        node_id: u64,
    },
    SetNodeSocketAddress {
        #[arg(short, long)]
        node_id: u64,
        #[arg(short, long)]
        socket_address: String,
    },
    GetNodeOwner {
        #[arg(short, long)]
        node_id: u64,
    },
}

impl CliOptions {
    // This is temporary until we move the execution of these to
    // the runtime.
    #[tokio::main]
    async fn rest_of_options<T: CliCommands>(command: Option<Commands>) -> Result<()> {
        match command {
            // cargo run --bin seda register-node --socket-address 127.0.0.1:9000
            Some(Commands::RegisterNode { socket_address }) => T::register_node(socket_address).await?,
            // cargo run --bin seda get-nodes --limit 2
            Some(Commands::GetNodes { limit, offset }) => T::get_nodes(limit, offset).await?,
            // cargo run --bin seda get-node-socket-address --node-id 9
            Some(Commands::GetNodeSocketAddress { node_id }) => T::get_node_socket_address(node_id).await?,
            // cargo run --bin seda remove-node --node-id 9
            Some(Commands::RemoveNode { node_id }) => T::remove_node(node_id).await?,
            // cargo run --bin seda set-node-socket-address --node-id 9 --socket-address 127.0.0.1:9000
            Some(Commands::SetNodeSocketAddress {
                node_id,
                socket_address,
            }) => T::set_node_socket_address(node_id, socket_address).await?,
            // cargo run --bin seda get-node-owner --node-id 9
            Some(Commands::GetNodeOwner { node_id }) => T::get_node_owner(node_id).await?,
            None => todo!(),
            // The commands `run` and `generate-config` are already handled.
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn handle<T: CliCommands>() -> Result<()> {
        let options = CliOptions::parse();
        dotenv::dotenv().ok();

        match options.command {
            Some(Commands::GenerateConfig) => return Config::create_template_from_path("./template_config.toml"),
            Some(Commands::Run { server_address }) => {
                let _config = Config::read_from_path(options.config_file.unwrap())?;
                seda_node::run::<T::MainChainAdapter>(&server_address);
                return Ok(());
            }
            _ => {}
        }
        let _config = Config::read_from_path(options.config_file.unwrap())?;

        Self::rest_of_options::<T>(options.command)
    }
}
