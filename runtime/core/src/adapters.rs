use std::{future::Future, process::Output};

pub trait DatabaseAdapter: Send {
    fn set(&mut self, key: &str, value: &str);
    fn get(&self, key: &str) -> Option<&String>;
}

pub trait HttpAdapter: Send {
    // TODO: add headers + methods
    fn fetch<F>(&mut self, url: &str) -> F where F: Future<Output = Result<reqwest::Response, reqwest::Error>>;
}

pub trait AdapterTypes: Clone + Default + 'static {
    type Database: DatabaseAdapter;
    type Http: HttpAdapter;
}

#[derive(Default)]
pub struct Adapters<Types>
where
    Types: AdapterTypes,
{
    pub database: Types::Database,
    pub http:     Types::Http,
}
