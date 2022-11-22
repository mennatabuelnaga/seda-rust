mod errors;

pub use errors::*;

// Todo #[cfg(test)]
pub mod test_adapter;
#[async_trait::async_trait]
pub trait HttpAdapter: Send {
    // TODO: add headers + methods
    async fn fetch(&mut self, url: &str) -> Result<String>;
}
