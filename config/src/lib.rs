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

#[cfg(not(target_family = "wasm"))]
fn config_path() -> PathBuf {
    let config_path = std::env::var("SEDA_CONFIG_PATH").unwrap_or_default();
    if !config_path.trim().is_empty() {
        Path::new(&config_path).to_path_buf()
    } else {
        Path::new(FULL_CONFIG_PATH).to_path_buf()
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn create_and_load_or_load_config() {
    let path = config_path();
    if !path.exists() {
        if let Err(err) = AppConfig::create_template_from_path(&path) {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
    let mut config = CONFIG.blocking_write();
    *config = match AppConfig::read_from_path(path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
}

#[cfg(not(target_family = "wasm"))]
lazy_static::lazy_static! {
    pub static ref CONFIG: Arc<RwLock<AppConfig>> = Arc::new(RwLock::new(Default::default()));
}
