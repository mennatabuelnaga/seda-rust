use std::{collections::HashMap, error::Error, fmt::Display, ops::Deref};

use crate::{Bytes, FromBytes, MemoryAdapter, ToBytes};

#[derive(Default)]
pub struct InMemory {
    memory: HashMap<String, Bytes>,
}

#[derive(Debug)]
struct AlreadyInserted(String);

impl Display for AlreadyInserted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Memory already contains key of `{}`", self.0)
    }
}

impl Error for AlreadyInserted {}

impl MemoryAdapter for InMemory {
    fn read<O>(&self, key: &str) -> Result<Option<O>, Box<dyn Error>>
    where
        O: FromBytes,
    {
        self.memory.get(key).map(|b| O::from_bytes(b.deref())).transpose()
    }

    fn write<V>(&mut self, key: &str, value: V) -> Result<(), Box<dyn Error>>
    where
        V: ToBytes,
    {
        if self.memory.contains_key(key) {
            Err(AlreadyInserted(key.into()))?
        }

        self.memory.insert(key.into(), value.to_bytes());
        Ok(())
    }
}
