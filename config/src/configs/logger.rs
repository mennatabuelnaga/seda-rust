use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialLoggerConfig {
    pub log_file_path: Option<PathBuf>,
}

impl PartialLoggerConfig {
    pub fn to_config(self, log_file_path: Option<&PathBuf>) -> LoggerConfig {
        LoggerConfig {
            log_file_path: self
                .log_file_path
                .unwrap_or(std::env::current_dir().expect("Failed to get current directory.")),
        }
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
