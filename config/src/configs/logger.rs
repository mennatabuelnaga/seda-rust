use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{env_overwrite, Config};

#[derive(Debug, Serialize, Deserialize)]
/// The configuration for the logger.
pub struct LoggerConfig {
    pub log_file_path: PathBuf,
}

pub struct PartialLoggerConfig {
    pub log_file_path: Option<PathBuf>,
}

impl PartialLoggerConfig {
    // I question the usage of cli args this way.
    // We should follow a more standard approach of just passing the args around.
    // i.e. cli flag is optional and we do deposit.unwrap_or(CONFIG.deposit);
    // Trait this?
    pub fn to_config(self) -> LoggerConfig {
        // TODO can we macro this?
        // Need to be able to differentiate between ones with defaults and not.
        // As well as ones with ENV vars and not...
        let log_file_path: PathBuf = if let Ok(str_path) = std::env::var("LOG_FILE_PATH") {
            // i think the overwrite_from_env trait should still exist but
            // moved to its own trait, we can clean up the macro to do all of em at once.
            str_path.into()
        } else if let Some(path) = self.log_file_path {
            path
        } else {
            // But what if it comes from the cli args??
            // UHHH
            // Let's write it out
            // Top level here example:
            // seda --log_file_path = "./" ...
            // let log_file_path =
            // log_file_path.as_ref().unwrap_or(&CONFIG.logging.log_file_path);
            // When does config go from partial to final?
            // How do we enforce a required value above? UGH
            // For this one we could probably give a default path.
            // But what about ones we can't?
            panic!("TODO should return an error bc this field is required.")
        };
        LoggerConfig { log_file_path }
    }
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
