// use crate::{DatabaseAdapter, HostAdapterTypes, RuntimeError, SuperHostAdapterTypes, SuperHttpAdapter};

// #[derive(Default, Clone)]
// pub struct SuperTestAdapter {}

// impl SuperHostAdapterTypes for SuperTestAdapter {
//     type Database = DatabaseSuperTestAdapter;
//     type Http = HttpSuperTestAdapter;
// }

// #[derive(Default, Clone)]
// pub struct DatabaseSuperTestAdapter {}

// #[async_trait::async_trait]
// impl DatabaseAdapter for DatabaseSuperTestAdapter {
//     async fn get(&self, key: &str) -> Result<Option<String>, RuntimeError> {
//         println!("CAlling get {}", key);
//         Ok(Some("hey".to_string()))
//     }

//     async fn set(&mut self, key: &str, value: &str) -> Result<(), RuntimeError> {
//         println!("CAlling set {} {}", key, value);
//         Ok(())
//     }
// }

// #[derive(Clone, Default)]
// pub struct HttpSuperTestAdapter;

// #[async_trait::async_trait]
// impl SuperHttpAdapter for HttpSuperTestAdapter {
//     async fn fetch(&mut self, url: &str) -> Result<String, ()> {
//         Ok("htttppppp".to_string())
//     }
// }
