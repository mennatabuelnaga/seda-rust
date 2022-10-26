use std::{
    collections::HashMap,
    fs,
    future::Future,
    process::{Command, Output},
    sync::{Arc, Mutex, Once},
};

use seda_runtime_macros::Adapter;

use super::{runtime::start_runtime, AdapterTypes, Adapters, DatabaseAdapter, VmConfig};
use crate::{adapters::HttpAdapter, RuntimeError};
#[derive(Clone, Default, Adapter)]
#[adapter(
    database = DatabaseTestAdapter,
    http = HttpTestAdapter,
)]
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

#[derive(Default, Clone)]
struct HttpTestAdapter {
    data: HashMap<String, String>,
}

impl HttpAdapter for HttpTestAdapter {
    fn fetch<F>(&mut self, url: &str) -> F
    where
        F: Future<Output = Result<reqwest::Response, reqwest::Error>>,
    {
        println!("---- Yaaaaayy-----");

        let x = reqwest::get(url);
        x
    }
}

static INIT: Once = Once::new();
// This is not a standard thing you can do in rust..
fn before_all() {
    INIT.call_once(|| {
        println!("Building WASM test binary...");

        let output = Command::new("cargo")
            .current_dir("./test_res/promise_wasm_bin")
            .args(["build", "--target", "wasm32-wasi", "--release"])
            .output();

        println!("Build completed: {}", output.is_ok());
    });
}

#[test]
fn test_promise_queue_multiple_calls_with_external_traits() {
    before_all();

    let wasm_binary = fs::read("../../target/wasm32-wasi/release/promise-wasm-bin.wasm").unwrap();

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

    let value = adapter_ref.database.get("test_value");

    assert!(runtime_execution_result.is_ok());
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "completed");
}

#[test]
fn test_bad_wasm_file() {
    before_all();

    let adapter = Arc::new(Mutex::new(Adapters::<TestAdapters>::default()));

    let runtime_execution_result = start_runtime(
        VmConfig {
            args:         vec!["hello world".to_string()],
            program_name: "consensus".to_string(),
            start_func:   None,
            wasm_binary:  vec![203],
            debug:        true,
        },
        adapter,
    );

    let error_type = match runtime_execution_result {
        Ok(_) => panic!("Runtime should error"),
        Err(err) => err,
    };

    match error_type {
        RuntimeError::WasmCompileError(_) => (),
        _ => panic!("WasmCompileError not triggered"),
    }
}

#[test]
fn test_non_existing_function() {
    before_all();

    let wasm_binary = fs::read("../../target/wasm32-wasi/release/promise-wasm-bin.wasm").unwrap();
    let adapter = Arc::new(Mutex::new(Adapters::<TestAdapters>::default()));

    let runtime_execution_result = start_runtime(
        VmConfig {
            args: vec!["hello world".to_string()],
            program_name: "consensus".to_string(),
            start_func: Some("non_existing_function".to_string()),
            wasm_binary,
            debug: true,
        },
        adapter,
    );

    let error_type = match runtime_execution_result {
        Ok(_) => panic!("Runtime should error"),
        Err(err) => err,
    };

    match error_type {
        RuntimeError::FunctionNotFound(_) => (),
        _ => panic!("FunctionNotFound not triggered"),
    }
}

#[test]
fn test_promise_queue_http_fetch() {
    before_all();

    let wasm_binary = fs::read("../../target/wasm32-wasi/release/promise-wasm-bin.wasm").unwrap();

    let adapter = Arc::new(Mutex::new(Adapters::<TestAdapters>::default()));

    let runtime_execution_result = start_runtime(
        VmConfig {
            args:         vec!["hello world".to_string()],
            program_name: "consensus".to_string(),
            start_func:   Some("http_fetch_test".to_string()),
            wasm_binary:  wasm_binary.to_vec(),
            debug:        true,
        },
        adapter.clone(),
    );
}
