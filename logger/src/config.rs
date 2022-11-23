use std::path::PathBuf;

use seda_config::{env_overwrite, Config};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The configuration for the logger.
pub struct LoggerConfig {
    pub log_file_path: Option<PathBuf>,
}

impl Config for LoggerConfig {
    fn template() -> Self {
        Self {
            log_file_path: Some(std::env::current_dir().expect("Failed to get current directory")),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.log_file_path, "LOG_FILE_PATH");
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        let mut this = Self { log_file_path: None };
        this.overwrite_from_env();
        this
    }
}
