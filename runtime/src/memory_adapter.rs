use std::error::Error;

use crate::{Bytes, FromBytes, ToBytes};

pub trait MemoryAdapter {
    fn get<V>(&self, key: &str) -> Result<Option<V>, Box<dyn Error>>
    where
        V: FromBytes;

    fn put<V>(&mut self, key: &str, value: V) -> Option<Bytes>
    where
        V: ToBytes;
}
