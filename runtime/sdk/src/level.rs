use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Level {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
}

impl Level {
    pub fn log(self, message: &str) {
        match self {
            Level::Debug => tracing::debug!(message),
            Level::Error => tracing::error!(message),
            Level::Info => tracing::info!(message),
            Level::Trace => tracing::trace!(message),
            Level::Warn => tracing::warn!(message),
        }
    }
}
