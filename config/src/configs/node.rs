use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{env_overwrite, merge_config_cli, Config, Result};

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
    pub rpc_server_address:      Option<String>,
    #[arg(long)]
    pub runtime_worker_threads:  Option<u8>,
    #[arg(long)]
    pub p2p_server_address:      Option<String>,
    #[arg(long)]
    pub p2p_known_peers:         Option<Vec<String>>,
}

impl PartialNodeConfig {
    pub fn to_config(self, cli_options: Self) -> Result<NodeConfig> {
        let deposit = merge_config_cli!(self, cli_options, deposit, NodeConfig::DEPOSIT, |f: String| f
            .parse()
            .unwrap());
        let gas = merge_config_cli!(self, cli_options, gas, NodeConfig::GAS);
        let secret_key = merge_config_cli!(self, cli_options, secret_key, panic!("todo"));
        // TODO this should be derived from the secret key?
        let public_key = merge_config_cli!(self, cli_options, public_key, panic!("todo"));
        let signer_account_id = merge_config_cli!(self, cli_options, signer_account_id, panic!("todo"));
        let contract_account_id = merge_config_cli!(self, cli_options, contract_account_id, panic!("todo"));
        let job_manager_interval_ms = merge_config_cli!(
            self,
            cli_options,
            job_manager_interval_ms,
            NodeConfig::JOB_MANAGER_INTERVAL_MS
        );
        let rpc_server_address = merge_config_cli!(self, cli_options, rpc_server_address, panic!("todo"));
        let runtime_worker_threads = merge_config_cli!(
            self,
            cli_options,
            runtime_worker_threads,
            NodeConfig::RUNTIME_WORKER_THREADS,
            |f| f as usize
        );
        let p2p_server_address =
            merge_config_cli!(self, cli_options, p2p_server_address, "/ip4/0.0.0.0/tcp/0".to_string());
        let p2p_known_peers = merge_config_cli!(self, cli_options, p2p_known_peers, Vec::new());

        Ok(NodeConfig {
            deposit,
            gas,
            secret_key,
            signer_account_id,
            contract_account_id,
            public_key,
            job_manager_interval_ms,
            rpc_server_address,
            runtime_worker_threads,
            p2p_server_address,
            p2p_known_peers,
        })
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
            rpc_server_address:      Some("127.0.0.1:12345".to_string()),
            runtime_worker_threads:  None,
            p2p_server_address:      Some("/ip4/0.0.0.0/tcp/0".to_string()),
            p2p_known_peers:         None,
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.secret_key, "SEDA_SECRET_KEY");
    }
}

// impl Default for PartialNodeConfig {
//     fn default() -> Self {
//         let mut this = Self {
//             deposit:                 Some((87 *
// 10_u128.pow(19)).to_string()),             gas:
// Some(300_000_000_000_000_u64.to_string()),             secret_key:
// Some("fill me in".to_string()),             signer_account_id:
// Some("fill me in".to_string()),             contract_account_id:
// Some("fill me in".to_string()),             public_key:
// Some("fill me in".to_string()),             job_manager_interval_ms:
// Some(10),             rpc_server_address:
// Some("127.0.0.1:12345".to_string()),             runtime_worker_threads:
// Some(2),             p2p_server_address:
// Some("/ip4/0.0.0.0/tcp/0".to_string()),             p2p_known_peers:
// None,         };
//         this.overwrite_from_env();
//         this
//     }
// }

#[derive(Debug)]
pub struct NodeConfig {
    pub deposit:                 u128,
    pub gas:                     u64,
    pub secret_key:              String,
    pub signer_account_id:       String,
    pub contract_account_id:     String,
    pub public_key:              String,
    pub job_manager_interval_ms: u64,
    pub rpc_server_address:      String,
    pub runtime_worker_threads:  usize,
    pub p2p_server_address:      String,
    pub p2p_known_peers:         Vec<String>,
}

impl NodeConfig {
    pub const DEPOSIT: u128 = 87 * 10_u128.pow(19);
    pub const GAS: u64 = 300_000_000_000_000;
    pub const JOB_MANAGER_INTERVAL_MS: u64 = 10;
    pub const RUNTIME_WORKER_THREADS: usize = 2;
}
