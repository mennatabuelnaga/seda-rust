use std::collections::HashMap;

pub trait DatabaseAdapter: Send {
    fn set(&mut self, key: &str, value: &str);
    fn get(&self, key: &str) -> Option<&String>;
    fn getAll(&self) -> HashMap<String, String>;
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

// pub trait HostAdapterTypes: Clone + Default + 'static {
//     type Database: DatabaseAdapter;
//     // type Http: HttpAdapter;
// }

// pub struct HostAdapters<HostTypes>
// where
//     HostTypes: HostAdapterTypes,
// {
//     pub database: HostTypes::Database,
//     // pub http:     HostTypes::Http,
// }

pub trait HostAdapterTypes: Send {
    type Database: DatabaseAdapter;
    type Http: HttpAdapter;
}

#[derive(Default)]
pub struct HostAdapters<T>
where
    T: HostAdapterTypes + Send,
{
    pub database: T::Database,
    pub http:     T::Http,
}
