use std::sync::Arc;

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{env_overwrite, merge_config_cli, Config, ConfigError, Result};

/// The configuration for the seda engine.
#[derive(Debug, Default, Serialize, Deserialize, Parser)]
pub struct PartialNodeConfig {
    #[arg(short, long)]
    pub deposit:                 Option<String>,
    #[arg(short, long)]
    pub gas:                     Option<u64>,
    #[arg(long)]
    pub secret_key:              Option<String>,
    #[arg(long)]
    pub signer_account_id:       Option<String>,
    #[arg(long)]
    pub contract_account_id:     Option<String>,
    #[arg(long)]
    pub public_key:              Option<String>,
    #[arg(long)]
    pub job_manager_interval_ms: Option<u64>,
    #[arg(long)]
    pub runtime_worker_threads:  Option<u8>,
    #[arg(long)]
    pub p2p_server_address:      Option<String>,
    #[arg(long)]
    pub p2p_known_peers:         Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
pub struct PartialDepositAndContractID {
    #[arg(short, long)]
    pub deposit:             Option<String>,
    #[arg(long)]
    pub contract_account_id: Option<String>,
}

/// A smaller configuration purely for other CLI commands
#[derive(Debug)]
pub struct DepositAndContractID {
    pub deposit:             u128,
    pub contract_account_id: String,
}

impl PartialNodeConfig {
    pub fn to_deposit_and_contract_id(self, cli_options: PartialDepositAndContractID) -> Result<DepositAndContractID> {
        let deposit = merge_config_cli!(self, cli_options, deposit, Ok(NodeConfigInner::DEPOSIT), |f: String| f
            .parse()
            .unwrap())?;
        let contract_account_id = merge_config_cli!(
            self,
            cli_options,
            contract_account_id,
            Err(ConfigError::from("node.contract_account_id"))
        )?;

        Ok(DepositAndContractID {
            deposit,
            contract_account_id,
        })
    }

    pub fn to_config(self, cli_options: Self) -> Result<NodeConfig> {
        let deposit = merge_config_cli!(self, cli_options, deposit, Ok(NodeConfigInner::DEPOSIT), |f: String| f
            .parse()
            .unwrap())?;
        let gas = merge_config_cli!(self, cli_options, gas, Ok(NodeConfigInner::GAS))?;
        let secret_key = merge_config_cli!(self, cli_options, secret_key, Err(ConfigError::from("node.secret_key")))?;
        // TODO this should be derived from the secret key
        let public_key = merge_config_cli!(self, cli_options, public_key, Err(ConfigError::from("node.public_key")))?;
        let signer_account_id = merge_config_cli!(
            self,
            cli_options,
            signer_account_id,
            Err(ConfigError::from("node.signer_account_id"))
        )?;
        let contract_account_id = merge_config_cli!(
            self,
            cli_options,
            contract_account_id,
            Err(ConfigError::from("node.contract_account_id"))
        )?;
        let job_manager_interval_ms = merge_config_cli!(
            self,
            cli_options,
            job_manager_interval_ms,
            Ok(NodeConfigInner::JOB_MANAGER_INTERVAL_MS)
        )?;
        let runtime_worker_threads = merge_config_cli!(
            self,
            cli_options,
            runtime_worker_threads,
            Ok(NodeConfigInner::RUNTIME_WORKER_THREADS),
            |f| f as usize
        )?;
        let p2p_server_address = merge_config_cli!(
            self,
            cli_options,
            p2p_server_address,
            Ok(NodeConfigInner::P2P_SERVER_ADDRESS.to_string())
        )?;
        let p2p_known_peers = merge_config_cli!(self, cli_options, p2p_known_peers, Ok(Vec::new()))?;

        Ok(Arc::new(NodeConfigInner {
            deposit,
            gas,
            secret_key,
            signer_account_id,
            contract_account_id,
            public_key,
            job_manager_interval_ms,
            runtime_worker_threads,
            p2p_server_address,
            p2p_known_peers,
        }))
    }
}

impl Config for PartialNodeConfig {
    fn template() -> Self {
        Self {
            deposit:                 None,
            gas:                     None,
            secret_key:              None,
            signer_account_id:       None,
            contract_account_id:     None,
            public_key:              None,
            job_manager_interval_ms: None,
            runtime_worker_threads:  None,
            p2p_server_address:      Some(NodeConfigInner::P2P_SERVER_ADDRESS.to_string()),
            p2p_known_peers:         None,
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.secret_key, "SEDA_SECRET_KEY");
    }
}

#[derive(Debug, Clone)]
pub struct NodeConfigInner {
    pub deposit:                 u128,
    pub gas:                     u64,
    pub secret_key:              String,
    pub signer_account_id:       String,
    pub contract_account_id:     String,
    pub public_key:              String,
    pub job_manager_interval_ms: u64,
    pub runtime_worker_threads:  usize,
    pub p2p_server_address:      String,
    pub p2p_known_peers:         Vec<String>,
}

impl NodeConfigInner {
    // TODO cfg this
    pub fn test_config() -> NodeConfig {
        Arc::new(Self {
            deposit:                 Self::DEPOSIT,
            gas:                     Self::GAS,
            secret_key:              String::new(),
            signer_account_id:       String::new(),
            contract_account_id:     String::new(),
            public_key:              String::new(),
            job_manager_interval_ms: Self::JOB_MANAGER_INTERVAL_MS,
            runtime_worker_threads:  Self::RUNTIME_WORKER_THREADS,
            p2p_server_address:      Self::P2P_SERVER_ADDRESS.to_string(),
            p2p_known_peers:         Vec::new(),
        })
    }
}

impl NodeConfigInner {
    pub const DEPOSIT: u128 = 87 * 10_u128.pow(19);
    pub const GAS: u64 = 300_000_000_000_000;
    pub const JOB_MANAGER_INTERVAL_MS: u64 = 10;
    pub const P2P_SERVER_ADDRESS: &str = "/ip4/0.0.0.0/tcp/0";
    pub const RUNTIME_WORKER_THREADS: usize = 2;
}

pub type NodeConfig = Arc<NodeConfigInner>;
