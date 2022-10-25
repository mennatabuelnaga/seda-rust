use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use seda_runtime_macros::Adapter;

use super::{runtime::start_runtime, AdapterTypes, Adapters, DatabaseAdapter, VmConfig};
#[derive(Clone, Default, Adapter)]
#[adapter(database = DatabaseTestAdapter)]
struct TestAdapters;

#[derive(Default, Clone)]
struct DatabaseTestAdapter {
    data: HashMap<String, String>,
}

impl DatabaseAdapter for DatabaseTestAdapter {
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

#[test]
fn start_runtime_simple() {
    let wasm_binary = include_bytes!("../../../target/wasm32-wasi/release/promise-wasm-bin.wasm");

    let adapter = Arc::new(Mutex::new(Adapters::<TestAdapters>::default()));

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

    let value = adapter_ref.database.get("from_wasm");

    assert!(runtime_execution_result.is_ok());
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "somevalue");
}
