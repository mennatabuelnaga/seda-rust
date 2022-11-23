use std::sync::Arc;

use tokio::sync::Mutex as AsyncMutex;

use super::RuntimeError;

#[async_trait::async_trait]
pub trait DatabaseAdapter: Send {
    async fn set(&mut self, key: &str, value: &str) -> Result<(), RuntimeError>;
    async fn get(&self, key: &str) -> Result<Option<String>, RuntimeError>;
}

#[async_trait::async_trait]
pub trait HttpAdapter: Send + 'static {
    // TODO: add headers + methods
    async fn fetch(&mut self, url: &str) -> Result<reqwest::Response, reqwest::Error>;
}

pub trait DummyAdapter: Clone + Default + 'static + Send + Sync {
    fn get(&self, key: &str);
}

pub trait VmAdapterTypes: Clone + Default + 'static + Send + Sync {
    // Put memory etc here, only sync traits
    type Dummy: DummyAdapter;
}

#[derive(Default)]
pub struct VmAdapters<VmTypes>
where
    VmTypes: VmAdapterTypes,
{
    pub memory: VmTypes::Dummy,
}
pub trait HostAdapterTypes: Default + Clone {
    type Database: DatabaseAdapter + Default + Clone;
    type Http: HttpAdapter + Default + Clone;
}

#[derive(Default, Clone)]
pub struct HostAdaptersInner<T>
where
    T: HostAdapterTypes,
{
    pub database: Arc<AsyncMutex<T::Database>>,
    pub http:     Arc<AsyncMutex<T::Http>>,
}

#[derive(Clone)]
pub struct HostAdapters<T>
where
    T: HostAdapterTypes,
{
    inner: HostAdaptersInner<T>,
}

impl<T> HostAdapters<T>
where
    T: HostAdapterTypes,
{
    pub fn new() -> Self {
        Self {
            inner: HostAdaptersInner::<T>::default(),
        }
    }

    pub fn db_get(&self, key: &str) -> Result<Option<String>, RuntimeError> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move { self.inner.database.lock().await.get(key).await })
        })
    }

    pub fn db_set(&self, key: &str, value: &str) -> Result<(), RuntimeError> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.inner.database.lock().await.set(key, value).await })
        })
    }

    pub fn http_fetch(&self, url: &str) -> Result<String, reqwest::Error> {
        let system = actix::System::current();
        let http = self.inner.http.clone();
        let url = url.to_string();
        let (sender, mut receiver) = tokio::sync::mpsc::channel(1);

        system.arbiter().spawn(async move {
            sender
                .send(match http.lock().await.fetch(&url).await {
                    Ok(something) => something.text().await,
                    Err(e) => Err(e),
                })
                .await
                .expect("Panicked: Could not received http fetch response: reason `channel failed`.");
        });

        receiver.blocking_recv().unwrap() // .ok_or(todo!("todo convert toour error type")
    }
}

impl<T> Default for HostAdapters<T>
where
    T: HostAdapterTypes,
{
    fn default() -> Self {
        Self::new()
    }
}
