use std::{collections::HashMap, error::Error, ops::Deref};

use crate::{Bytes, FromBytes, MemoryAdapter, ToBytes};

#[derive(Default)]
pub struct InMemory {
    memory: HashMap<String, Bytes>,
}

impl MemoryAdapter for InMemory {
    fn get<O>(&self, key: &str) -> Result<Option<O>, Box<dyn Error>>
    where
        O: FromBytes,
    {
        self.memory.get(key).map(|b| O::from_bytes(b.deref())).transpose()
    }

    fn put<V>(&mut self, key: &str, value: V) -> Option<Bytes>
    where
        V: ToBytes,
    {
        self.memory.insert(key.into(), value.to_bytes())
    }
}
