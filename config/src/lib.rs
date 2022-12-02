mod config;
pub use config::*;

mod configs;
pub use configs::*;

mod errors;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub use errors::*;
use tokio::sync::RwLock;

pub const FULL_CONFIG_PATH: &str = "/etc/seda-rust.toml";

lazy_static::lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = {
        let config_path = std::env::var("SEDA_CONFIG_PATH").unwrap_or_default();
                if !config_path.trim().is_empty() {
                    Path::new(&config_path).to_path_buf()
                }
                else {
                    Path::new(FULL_CONFIG_PATH).to_path_buf()
                }
    };

    // TODO we should conditionally compile this with the right generic.
    pub static ref CONFIG: Arc<RwLock<AppConfig>> = {
            let path = CONFIG_PATH.to_path_buf();
            if !path.exists() {
                            if let Err(err) = AppConfig::create_template_from_path(&path) {
                                eprintln!("{err}");
                                std::process::exit(1);
                            }
            }
            match AppConfig::read_from_path(path) {
                Ok(config) => Arc::new(RwLock::new(config)),
                Err(err) => {
                                    eprintln!("{err}");
                                    std::process::exit(1);
                }
            }

    };
}
