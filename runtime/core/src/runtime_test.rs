use std::{fs, process::Command, sync::Once};

use super::VmConfig;
use crate::{
    adapters::HostAdapters,
    runtime::{RunnablePotato, Runtime},
    test::test_adapters::TestAdapters,
    RuntimeError,
};

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

#[tokio::test]
async fn test_promise_queue_multiple_calls_with_external_traits() {
    before_all();

    let wasm_binary = fs::read("../../target/wasm32-wasi/release/promise-wasm-bin.wasm").unwrap();
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime.start_runtime(
        VmConfig {
            args:         vec!["hello world".to_string()],
            program_name: "consensus".to_string(),
            start_func:   None,
            wasm_binary:  wasm_binary.to_vec(),
            debug:        true,
        },
        host_adapter.clone(),
    );
    assert!(runtime_execution_result.await.is_ok());

    let value = host_adapter.db_get("test_value");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "completed");
}

#[tokio::test]
async fn test_bad_wasm_file() {
    before_all();

    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args:         vec!["hello world".to_string()],
                program_name: "consensus".to_string(),
                start_func:   None,
                wasm_binary:  vec![203],
                debug:        true,
            },
            host_adapter.clone(),
        )
        .await;

    assert!(runtime_execution_result.is_err());

    let error_type = match runtime_execution_result {
        Ok(_) => panic!("Runtime should error"),
        Err(err) => err,
    };

    match error_type {
        RuntimeError::WasmCompileError(_) => (),
        _ => panic!("WasmCompileError not triggered"),
    }
}

#[tokio::test]
async fn test_non_existing_function() {
    before_all();

    let wasm_binary = fs::read("../../target/wasm32-wasi/release/promise-wasm-bin.wasm").unwrap();
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args: vec!["hello world".to_string()],
                program_name: "consensus".to_string(),
                start_func: Some("non_existing_function".to_string()),
                wasm_binary,
                debug: true,
            },
            host_adapter.clone(),
        )
        .await;

    let error_type = match runtime_execution_result {
        Ok(_) => panic!("Runtime should error"),
        Err(err) => err,
    };

    match error_type {
        RuntimeError::FunctionNotFound(_) => (),
        _ => panic!("FunctionNotFound not triggered"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_promise_queue_http_fetch() {
    before_all();

    let wasm_binary = fs::read("../../target/wasm32-wasi/release/promise-wasm-bin.wasm").unwrap();
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args:         vec!["hello world".to_string()],
                program_name: "consensus".to_string(),
                start_func:   Some("http_fetch_test".to_string()),
                wasm_binary:  wasm_binary.to_vec(),
                debug:        true,
            },
            host_adapter.clone(),
        )
        .await;

    assert!(runtime_execution_result.is_ok());
}
