use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The configuration for the logger.
pub struct LoggerConfig {
    pub log_file_path: PathBuf,
}

impl Config for LoggerConfig {
    fn template() -> Self {
        Self {
            log_file_path: std::env::current_dir().expect("Failed to get current directory"),
        }
    }

    fn overwrite_from_env(&mut self) {
        env_overwrite!(self.log_file_path, "LOG_FILE_PATH");
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        let mut this = Self {
            log_file_path: std::env::current_dir().expect("Failed to get current directory"),
        };
        this.overwrite_from_env();
        this
    }
}
