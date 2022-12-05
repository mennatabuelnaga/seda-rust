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

// Standard config location for unix apps.
#[cfg(target_family = "unix")]
pub const FULL_CONFIG_PATH: &str = "/etc/seda-rust/config.toml";
// Standard config location for windows apps.
#[cfg(target_family = "windows")]
pub const FULL_CONFIG_PATH: &str = "C:\\ProgramData\\seda-rust\\config.toml";

fn config_path() -> PathBuf {
    let config_path = std::env::var("SEDA_CONFIG_PATH").unwrap_or_default();
    if !config_path.trim().is_empty() {
        Path::new(&config_path).to_path_buf()
    } else {
        Path::new(FULL_CONFIG_PATH).to_path_buf()
    }
}

// The logger crate is not used here since
// the logger crate depends on this crate.
// Therefore logging isn't loaded until we
// read some settings from the config file.
fn create_and_load_or_load_config() -> Arc<RwLock<AppConfig>> {
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
}

lazy_static::lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = config_path();
    pub static ref CONFIG: Arc<RwLock<AppConfig>> = create_and_load_or_load_config();
}
