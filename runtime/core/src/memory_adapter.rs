use super::{Bytes, FromBytes, Result, ToBytes};

pub trait MemoryAdapter: Send {
    fn get<V>(&self, key: &str) -> Result<Option<V>>
    where
        V: FromBytes;

    fn put<V>(&mut self, key: &str, value: V) -> Option<Bytes>
    where
        V: ToBytes;
}
