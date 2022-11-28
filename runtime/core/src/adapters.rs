use super::RuntimeError;

#[async_trait::async_trait]
pub trait HostAdapter: Send {
    async fn db_get(key: &str) -> Result<Option<String>, RuntimeError>;
    async fn db_set(key: &str, value: &str) -> Result<(), RuntimeError>;
    async fn http_fetch(url: &str) -> Result<String, RuntimeError>;
}
