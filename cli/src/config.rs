use seda_adapters::MainChainAdapterTrait;
use seda_config::{env_overwrite, Config};
use serde::{Deserialize, Serialize};

use crate::errors::{CliError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig<T: MainChainAdapterTrait> {
    // todo these should be optional then
    deposit_for_register_node: String,
    gas:                       u64,
    secret_key:                String,
    signer_account_id:         String,
    contract_account_id:       String,
    public_key:                String,
    seda_server_url:           String,

    // TODO better name main_chain_config to appropriate
    // mainchain name. Can be done once we do conditional
    // compilation to select mainchain
    main_chain_config: Option<T::Config>,
    node_config:       Option<seda_node::NodeConfig>,
}

impl<T: MainChainAdapterTrait> Default for AppConfig<T> {
    fn default() -> Self {
        Self {
            node_config:               Some(Default::default()),
            deposit_for_register_node: (87 * 10_u128.pow(19)).to_string(),
            gas:                       300_000_000_000_000,
            secret_key:                "fill me in".to_string(),
            signer_account_id:         "fill me in".to_string(),
            contract_account_id:       "fill me in".to_string(),
            public_key:                "fill me in".to_string(),
            seda_server_url:           "fill me in".to_string(),
            main_chain_config:         Some(Default::default()),
        }
    }
}

impl<T: MainChainAdapterTrait> Config for AppConfig<T> {
    type Error = crate::errors::CliError;

    fn validate(&self) -> Result<(), Self::Error> {
        todo!()
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.seda_server_url, "SEDA_SERVER_URL");
        env_overwrite!(self.signer_account_id, "SIGNER_ACCOUNT_ID");
        env_overwrite!(self.secret_key, "SECRET_KEY");
        env_overwrite!(self.contract_account_id, "CONTRACT_ACCOUNT_ID");
        if let Some(main_chain_config) = self.main_chain_config.as_mut() {
            main_chain_config.overwrite_from_env()
        }
        if let Some(node_config) = self.node_config.as_mut() {
            node_config.overwrite_from_env()
        }
        todo!()
    }
}

impl<T: MainChainAdapterTrait> AppConfig<T> {
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
