use std::error::Error;

use crate::{FromBytes, ToBytes};

pub trait MemoryAdapter {
    fn read<V>(&self, key: &str) -> Result<Option<V>, Box<dyn Error>>
    where
        V: FromBytes;

    fn write<V>(&mut self, key: &str, value: V) -> Result<(), Box<dyn Error>>
    where
        V: ToBytes;
}
