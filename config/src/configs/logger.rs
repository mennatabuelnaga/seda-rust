use std::path::PathBuf;

use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{env_overwrite, merge_config_cli, Config, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Args)]
pub struct PartialLoggerConfig {
    #[arg(long)]
    pub log_file_path: Option<PathBuf>,
}

impl PartialLoggerConfig {
    pub fn to_config(self, cli_options: Self) -> Result<LoggerConfig> {
        let log_file_path = merge_config_cli!(
            self,
            cli_options,
            log_file_path,
            std::env::current_dir().map_err(|e| crate::ConfigError::FailedToGetCurrentDir(e.to_string()))
        )?;
        Ok(LoggerConfig { log_file_path })
    }
}

impl Config for PartialLoggerConfig {
    fn template() -> Self {
        Self {
            log_file_path: std::env::current_dir().ok(),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.log_file_path, "SEDA_LOG_FILE_PATH", |p| Some(PathBuf::from(p)));
    }
}

/// The configuration for the logger.
#[derive(Debug)]
pub struct LoggerConfig {
    pub log_file_path: PathBuf,
}
