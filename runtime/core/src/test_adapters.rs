use std::collections::HashMap;

use crate::adapters::{DatabaseAdapter, HostAdapterTypes};

#[derive(Default)]
pub struct TestAdapters;

#[derive(Default)]
pub struct DatabaseTestAdapter {
    data: HashMap<String, String>,
}

impl DatabaseAdapter for DatabaseTestAdapter {
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    fn getAll(&self) -> HashMap<String, String> {
        self.data.clone()
    }
}

impl HostAdapterTypes for TestAdapters {
    type Database = DatabaseTestAdapter;
}
