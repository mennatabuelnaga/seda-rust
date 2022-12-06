use rusqlite::params;
use tokio_rusqlite::Connection;

use crate::{DatabaseAdapter, Result, StorageAdapterError};

#[derive(Clone)]
pub struct DatabaseTestAdapter {
}

impl Default for DatabaseTestAdapter {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl DatabaseAdapter for DatabaseTestAdapter {
    async fn get(&self, key: &str) -> Result<Option<String>, RuntimeError> {
        Ok(Some("".to_string()))
    }

    async fn set(&mut self, key: &str, value: &str) -> Result<(), RuntimeError> {
        Ok(())
    }
}
