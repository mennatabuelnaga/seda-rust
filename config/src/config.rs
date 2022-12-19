use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    env_overwrite,
    errors::{Result, TomlError},
    AnotherConfig,
    Config,
    LoggerConfig,
    NearConfig,
    NodeConfig,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub seda_server_url: String,

    pub another_chain: AnotherConfig,
    pub near_chain:    NearConfig,
    pub node:          NodeConfig,
    pub logging:       LoggerConfig,
}

impl AsRef<AppConfig> for AppConfig {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut this = Self {
            seda_server_url: "ws://127.0.0.1:12345".to_string(),
            node:            Default::default(),
            another_chain:   AnotherConfig::default(),
            near_chain:      NearConfig::default(),
            logging:         Default::default(),
        };
        this.overwrite_from_env();
        this
    }
}

impl Config for AppConfig {
    fn template() -> Self {
        Self {
            seda_server_url: "ws://127.0.0.1:12345".to_string(),
            node:            NodeConfig::template(),
            another_chain:   AnotherConfig::template(),
            near_chain:      NearConfig::template(),
            logging:         LoggerConfig::template(),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.seda_server_url, "SEDA_SERVER_URL");
        self.another_chain.overwrite_from_env();
        self.near_chain.overwrite_from_env();
        self.node.overwrite_from_env();
        self.logging.overwrite_from_env();
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
