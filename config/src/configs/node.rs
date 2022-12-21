use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config};
#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
/// The configuration for the seda engine.
pub struct NodeConfig {
    pub deposit:                 String,
    pub gas:                     String,
    pub secret_key:              String,
    pub signer_account_id:       String,
    pub contract_account_id:     String,
    pub public_key:              String,
    // TODO should this be overwritten from
    // env and cli?
    pub job_manager_interval_ms: u64,
    pub rpc_server_address:      String,
    pub runtime_worker_threads:  usize,
    pub p2p_server_address:      String,
    pub p2p_known_peers:         Vec<String>,
}

impl Config for NodeConfig {
    fn template() -> Self {
        Self {
            deposit:                 (87 * 10_u128.pow(19)).to_string(),
            gas:                     300_000_000_000_000_u64.to_string(),
            secret_key:              "fill me in".to_string(),
            signer_account_id:       "fill me in".to_string(),
            contract_account_id:     "fill me in".to_string(),
            public_key:              "fill me in".to_string(),
            job_manager_interval_ms: 10,
            rpc_server_address:      "127.0.0.1:12345".to_string(),
            runtime_worker_threads:  2,
            p2p_server_address:      "/ip4/0.0.0.0/tcp/0".to_string(),
            p2p_known_peers:         vec![],
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.deposit, "DEPOSIT");
        env_overwrite!(self.gas, "GAS");
        env_overwrite!(self.signer_account_id, "SIGNER_ACCOUNT_ID");
        env_overwrite!(self.secret_key, "SECRET_KEY");
        env_overwrite!(self.contract_account_id, "CONTRACT_ACCOUNT_ID");
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        let mut this = Self {
            deposit:                 (87 * 10_u128.pow(19)).to_string(),
            gas:                     300_000_000_000_000_u64.to_string(),
            secret_key:              "fill me in".to_string(),
            signer_account_id:       "fill me in".to_string(),
            contract_account_id:     "fill me in".to_string(),
            public_key:              "fill me in".to_string(),
            job_manager_interval_ms: 10,
            rpc_server_address:      "127.0.0.1:12345".to_string(),
            runtime_worker_threads:  2,
            p2p_server_address:      "/ip4/0.0.0.0/tcp/0".to_string(),
            p2p_known_peers:         vec![],
        };
        this.overwrite_from_env();
        this
    }
}
