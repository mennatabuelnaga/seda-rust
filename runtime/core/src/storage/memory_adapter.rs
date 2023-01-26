use seda_config::NodeConfig;

use super::{Bytes, FromBytes, ToBytes};
use crate::Result;

pub trait MemoryAdapter: Default {
    const CONFIG_KEY: &'static str = "*&_seda_node_config";

    fn new(node_config: &NodeConfig) -> Result<Self>
    where
        Self: Sized,
    {
        let mut memory = Self::default();
        let config_str = serde_json::to_string(node_config)?;
        memory.put(Self::CONFIG_KEY, config_str);
        Ok(memory)
    }

    fn get_config(&self) -> Result<NodeConfig> {
        // Can safely unwrap here
        let config: String = self.get(Self::CONFIG_KEY)?.unwrap();
        Ok(serde_json::from_str(&config)?)
    }

    fn get<V>(&self, key: &str) -> Result<Option<V>>
    where
        V: FromBytes;

    fn put<V>(&mut self, key: &str, value: V) -> Option<Bytes>
    where
        V: ToBytes;
}
