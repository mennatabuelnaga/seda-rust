mod errors;
pub use errors::*;

// Todo #[cfg(test)]
pub mod test_adapter;

#[async_trait::async_trait]
pub trait DatabaseAdapter: Send {
    async fn set(&mut self, key: &str, value: &str) -> Result<()>;
    async fn get(&self, key: &str) -> Result<Option<String>>;
}
