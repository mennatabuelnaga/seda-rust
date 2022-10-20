use std::collections::HashMap;

use crate::adapters::DatabaseAdapter;

struct DatabaseTestAdapter {
    data: HashMap<String, String>,
}

impl DatabaseAdapter for DatabaseTestAdapter {
    fn get(&self, key: &str) {
        let data = self.data.get(key);

        println!("From database {:?}", &data);
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        println!("Called set for the database");
    }
}

#[cfg(test)]
mod runtime_tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use super::DatabaseTestAdapter;
    use crate::{adapters::Adapters, config::VmConfig, runtime::start_runtime};

    #[test]
    fn start_runtime_simple() {
        let wasm_binary = include_bytes!("../../target/wasm32-wasi/release/promise-wasm-bin.wasm");

        let adapter = Arc::new(Mutex::new(Adapters {
            database: Box::new(DatabaseTestAdapter { data: HashMap::new() }),
        }));

        let runtime_execution_result = start_runtime(
            VmConfig {
                args:         vec!["hello world".to_string()],
                program_name: "consensus".to_string(),
                start_func:   None,
                wasm_binary:  wasm_binary.to_vec(),
                debug:        true,
            },
            adapter.clone(),
        );

        let adapter_ref = adapter.lock().unwrap();
        adapter_ref.database.get("key");

        assert!(runtime_execution_result.is_ok())
    }
}
