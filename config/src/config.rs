use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    env_overwrite,
    errors::{Result, TomlError},
    Config,
    LoggerConfig,
    MainChainConfig,
    NodeConfig,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub seda_server_url: Option<String>,

    pub main_chain: Option<MainChainConfig>,
    pub node:       Option<NodeConfig>,
    pub logging:    Option<LoggerConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut this = Self {
            seda_server_url: None,
            node:            Some(Default::default()),
            main_chain:      Some(MainChainConfig::default()),
            logging:         Some(Default::default()),
        };
        this.overwrite_from_env();
        this
    }
}

impl Config for AppConfig {
    fn template() -> Self {
        Self {
            seda_server_url: Some("fill me in".to_string()),
            node:            Some(NodeConfig::template()),
            main_chain:      Some(MainChainConfig::template()),
            logging:         Some(LoggerConfig::template()),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.seda_server_url, "SEDA_SERVER_URL");
        if let Some(main_chain_config) = self.main_chain.as_mut() {
            main_chain_config.overwrite_from_env()
        }
        if let Some(node_config) = self.node.as_mut() {
            node_config.overwrite_from_env()
        }
        if let Some(logging_config) = self.logging.as_mut() {
            logging_config.overwrite_from_env()
        }
    }
}

impl AppConfig {
    /// For reading from a toml file.
    pub fn from_read<R: std::io::Read>(buf: &mut R) -> Result<Self> {
        let mut content = String::new();
        buf.read_to_string(&mut content)?;
        let mut config: Self = toml::from_str(&content).map_err(TomlError::Deserialize)?;
        config.overwrite_from_env();
        Ok(config)
    }

    /// For reading from a toml file from a path.
    pub fn read_from_path(path: PathBuf) -> Result<Self> {
        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        Self::from_read(&mut file)
    }

    /// For writing a default configuration file.
    pub fn write_template<W: std::io::Write>(buf: &mut W) -> Result<()> {
        let template = Self::template();
        let content = toml::to_string_pretty(&template).map_err(TomlError::Serialize)?;
        buf.write_all(content.as_bytes())?;
        Ok(())
    }

    /// For creating a default config to a given path.
    pub fn create_template_from_path(path: &PathBuf) -> Result<()> {
        if let Some(prefix) = path.parent() {
            if !prefix.exists() {
                std::fs::create_dir_all(prefix)?;
            }
        }
        let mut file = std::fs::OpenOptions::new().create(true).write(true).open(path)?;
        Self::write_template(&mut file)
    }
}
