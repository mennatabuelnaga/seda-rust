use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub trait DatabaseAdapter: Send {
    fn set(&mut self, key: &str, value: &str);
    fn get(&self, key: &str) -> Option<&String>;
    fn get_all(&self) -> HashMap<String, String>;
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
    type Database: DatabaseAdapter + Default;
    type Http: HttpAdapter + Default;
}

#[derive(Default, Clone)]
pub struct HostAdaptersInner<T>
where
    T: HostAdapterTypes,
{
    pub database: T::Database,
    pub http:     T::Http,
}

#[derive(Clone)]
pub struct HostAdapters<T>
where
    T: HostAdapterTypes,
{
    inner: Arc<Mutex<HostAdaptersInner<T>>>,
}

impl<T> HostAdapters<T>
where
    T: HostAdapterTypes,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HostAdaptersInner::<T>::default())),
        }
    }

    pub fn db_get(&self, key: &str) -> Option<String> {
        self.inner.lock().unwrap().database.get(key).cloned()
    }

    pub fn db_set(&self, key: &str, value: &str) {
        self.inner.lock().as_mut().unwrap().database.set(key, value);
    }

    pub fn http_fetch(&self, url: &str) -> Result<String, reqwest::Error> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.inner.lock().as_mut().unwrap().http.fetch(url).await?.text().await })
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
