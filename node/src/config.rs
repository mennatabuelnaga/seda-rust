use seda_config::{env_overwrite, Config};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// The configuration for the seda engine.

#[derive(Clone)]
pub struct NodeConfig {
    pub deposit_for_register_node: Option<String>,
    pub gas:                       Option<String>,
    pub secret_key:                Option<String>,
    pub signer_account_id:         Option<String>,
    pub contract_account_id:       Option<String>,
    pub public_key:                Option<String>,
    // TODO should this be overwritten from
    // env and cli?
    pub job_manager_interval_ms:   u64,
    pub server_address:            Option<String>,
    pub runtime_worker_threads:    u32,
}

impl Config for NodeConfig {
    fn template() -> Self {
        Self {
            deposit_for_register_node: Some((87 * 10_u128.pow(19)).to_string()),
            gas:                       Some(300_000_000_000_000_u64.to_string()),
            secret_key:                Some("fill me in".to_string()),
            signer_account_id:         Some("fill me in".to_string()),
            contract_account_id:       Some("fill me in".to_string()),
            public_key:                Some("fill me in".to_string()),
            job_manager_interval_ms:   10,
            server_address:            Some("fill me in".to_string()),
            runtime_worker_threads:    2,
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.deposit_for_register_node, "DEPOSIT");
        env_overwrite!(self.gas, "GAS");
        env_overwrite!(self.signer_account_id, "SIGNER_ACCOUNT_ID");
        env_overwrite!(self.secret_key, "SECRET_KEY");
        env_overwrite!(self.contract_account_id, "CONTRACT_ACCOUNT_ID");
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        let mut this = Self {
            deposit_for_register_node: None,
            gas:                       None,
            secret_key:                None,
            signer_account_id:         None,
            contract_account_id:       None,
            public_key:                None,
            job_manager_interval_ms:   10,
            server_address:            None,
            runtime_worker_threads:    2,
        };
        this.overwrite_from_env();
        this
    }
}
