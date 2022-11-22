use crate::{HttpAdapter, Result};

#[derive(Clone, Default)]
pub struct HttpTestAdapter;

#[async_trait::async_trait]
impl HttpAdapter for HttpTestAdapter {
    async fn fetch(&mut self, url: &str) -> Result<String> {
        Ok(reqwest::get(url).await?.text().await?)
    }
}
