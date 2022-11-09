use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::Mutex as AsyncMutex;


pub trait DatabaseAdapter: Send {
    // fn set(&mut self, key: &str, value: &str);
    fn set(&mut self, key: &str, value: &str) -> Result<(), rusqlite::Error>;
    // fn get(&self, key: &str) -> Option<&String>;
    fn get(&self, key: &str) ->Result<Option<String>, rusqlite::Error>;
    // fn get_all(&self) -> HashMap<String, String>;
}

#[async_trait::async_trait]
pub trait HttpAdapter: Send {
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
    pub database: Arc<Mutex<T::Database>>,
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

  
    pub fn db_get(&self, key: &str) -> Result<Option<String>, rusqlite::Error>{
        self.inner.database.lock().get(key)

    }
   

    pub fn db_set(&self, key: &str, value: &str) -> Result<(), rusqlite::Error>{
        self.inner.database.lock().set(key, value)
    }
   
    pub fn http_fetch(&self, url: &str) -> Result<String, reqwest::Error> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.inner.http.lock().await.fetch(url).await?.text().await })
        })
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
