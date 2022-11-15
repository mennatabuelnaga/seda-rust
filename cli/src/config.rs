use serde::{Deserialize, Serialize};

use crate::errors::{CliError, Result};

#[derive(Serialize, Deserialize)]
pub struct Config {
    // Node Config only necessary when the cli launches the node?
    // node_config: seda_node::NodeConfig,
    // register_options: RegisterNodeOptions,

    // TODO consider not toml cause we can't use u128s
    deposit_for_register_node: String,
    gas:                       u64,
    secret_key:                String,
    signer_account_id:         String,
    contract_account_id:       String,
    public_key:                String,
    seda_server_url:           String,
    // TODO move to Near Mainchain Adapter impl
    near_server_url:           String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            deposit_for_register_node: (87 * 10_u128.pow(19)).to_string(),
            gas:                       300_000_000_000_000,
            secret_key:                "fill me in".to_string(),
            signer_account_id:         "fill me in".to_string(),
            contract_account_id:       "fill me in".to_string(),
            public_key:                "fill me in".to_string(),
            seda_server_url:           "fill me in".to_string(),
            near_server_url:           "fill me in".to_string(),
        }
    }
}

macro_rules! env_overwrite {
    ($field:expr, $name:expr) => {
        if let Ok(var) = std::env::var($name) {
            $field = var;
        }
    };
}

impl Config {
    // TODO should we enforce that all config values exist
    // then overwrite
    // or should we just read config values
    // then overwrite but not enforce
    // or some combination of both
    pub fn verify(&self) -> Result<()> {
        todo!()
    }

    pub fn overwrite_from_env(&mut self) {
        env_overwrite!(self.seda_server_url, "SEDA_SERVER_URL");
        env_overwrite!(self.near_server_url, "NEAR_SERVER_URL");
        env_overwrite!(self.signer_account_id, "SIGNER_ACCOUNT_ID");
        env_overwrite!(self.secret_key, "SECRET_KEY");
        env_overwrite!(self.contract_account_id, "CONTRACT_ACCOUNT_ID");
    }

    /// For reading from a toml file.
    pub fn from_read<R: std::io::Read>(buf: &mut R) -> Result<Self> {
        let mut content = String::new();
        buf.read_to_string(&mut content)?;
        let mut config: Self = toml::from_str(&content).map_err(|err| CliError::InvalidTomlConfig(err.to_string()))?;
        config.overwrite_from_env();
        Ok(config)
    }

    /// For reading from a toml file from a path.
    pub fn read_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        Self::from_read(&mut file)
    }

    /// For writing a default configuration file.
    pub fn write_template<W: std::io::Write>(buf: &mut W) -> Result<()> {
        let template = Self::default();
        let content = toml::to_string_pretty(&template).map_err(|err| CliError::InvalidTomlConfig(err.to_string()))?;
        buf.write_all(content.as_bytes())?;
        Ok(())
    }

    /// For creating a default config to a given path.
    pub fn create_template_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<()> {
        let mut file = std::fs::OpenOptions::new().create(true).write(true).open(path)?;
        Self::write_template(&mut file)
    }
}
