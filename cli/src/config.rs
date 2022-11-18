use seda_adapters::MainChainAdapterTrait;
use seda_config::Config;
use seda_node::NodeConfig;
use serde::{Deserialize, Serialize};

use crate::errors::{CliError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig<T: MainChainAdapterTrait> {
    // todo these should be optional then
    pub deposit_for_register_node: String,
    pub gas:                       u64,
    pub secret_key:                Option<String>,
    pub signer_account_id:         Option<String>,
    pub contract_account_id:       Option<String>,
    pub public_key:                Option<String>,
    pub seda_server_url:           Option<String>,

    // TODO better name main_chain_config to appropriate
    // mainchain name. Can be done once we do conditional
    // compilation to select mainchain
    pub main_chain_config: Option<T::Config>,
    pub node_config:       Option<seda_node::NodeConfig>,
}

impl<T: MainChainAdapterTrait> Default for AppConfig<T> {
    fn default() -> Self {
        let mut this = Self {
            node_config:               Some(Default::default()),
            deposit_for_register_node: (87 * 10_u128.pow(19)).to_string(),
            gas:                       300_000_000_000_000,
            secret_key:                None,
            signer_account_id:         None,
            contract_account_id:       None,
            public_key:                None,
            seda_server_url:           None,
            main_chain_config:         Some(Default::default()),
        };
        this.overwrite_from_env();
        this
    }
}

impl<T: MainChainAdapterTrait> Config for AppConfig<T> {
    type Error = crate::errors::CliError;

    fn validate(&self) -> Result<(), Self::Error> {
        todo!()
    }

    fn template() -> Self {
        Self {
            node_config:               Some(NodeConfig::template()),
            deposit_for_register_node: (87 * 10_u128.pow(19)).to_string(),
            gas:                       300_000_000_000_000,
            secret_key:                Some("fill me in".to_string()),
            signer_account_id:         Some("fill me in".to_string()),
            contract_account_id:       Some("fill me in".to_string()),
            public_key:                Some("fill me in".to_string()),
            seda_server_url:           Some("fill me in".to_string()),
            main_chain_config:         Some(T::Config::template()),
        }
    }

    fn overwrite_from_env(&mut self) {
        self.seda_server_url = std::env::var("SEDA_SERVER_URL").ok();
        self.signer_account_id = std::env::var("SIGNER_ACCOUNT_ID").ok();
        self.secret_key = std::env::var("SECRET_KEY").ok();
        self.contract_account_id = std::env::var("CONTRACT_ACCOUNT_ID").ok();
        if let Some(main_chain_config) = self.main_chain_config.as_mut() {
            main_chain_config.overwrite_from_env()
        }
        if let Some(node_config) = self.node_config.as_mut() {
            node_config.overwrite_from_env()
        }
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

    /// For reading from a toml file from a path if the path exists.
    /// Otherwise it returns a default object.
    pub fn read_from_optional_path<P: AsRef<std::path::Path>>(path: Option<P>) -> Result<Self> {
        match path {
            Some(path) => Self::read_from_path(path),
            None => Ok(Self::default()),
        }
    }

    /// For writing a default configuration file.
    pub fn write_template<W: std::io::Write>(buf: &mut W) -> Result<()> {
        let template = Self::template();
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
