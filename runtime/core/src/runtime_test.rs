use std::{fs, process::Command, sync::Once};

use super::VmConfig;
use crate::{
    adapters::HostAdapters,
    promise::PromiseStatus,
    runtime::{RunnablePotato, Runtime},
    test::test_adapters::TestAdapters,
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
            args: vec!["hello world".to_string()],
            program_name: "consensus".to_string(),
            start_func: None,
            wasm_binary: wasm_binary.to_vec(),
            debug: true,
        },
        host_adapter.clone(),
    );
    assert!(runtime_execution_result.await.is_ok());

    let value = host_adapter.db_get("test_value");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "completed");
}

#[tokio::test]
#[should_panic(expected = "input bytes aren't valid utf-8")]
async fn test_bad_wasm_file() {
    before_all();

    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args: vec!["hello world".to_string()],
                program_name: "consensus".to_string(),
                start_func: None,
                wasm_binary: vec![203],
                debug: true,
            },
            host_adapter.clone(),
        )
        .await;

    assert!(runtime_execution_result.is_err());
    runtime_execution_result.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Missing export non_existing_function")]
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

    assert!(runtime_execution_result.is_err());
    runtime_execution_result.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_promise_queue_http_fetch() {
    let fetch_url = "https://swapi.dev/api/people/2/".to_string();

    before_all();

    let wasm_binary = fs::read("../../target/wasm32-wasi/release/promise-wasm-bin.wasm").unwrap();
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args: vec![fetch_url.clone()],
                program_name: "consensus".to_string(),
                start_func: Some("http_fetch_test".to_string()),
                wasm_binary: wasm_binary.to_vec(),
                debug: true,
            },
            host_adapter.clone(),
        )
        .await;
    assert!(runtime_execution_result.is_ok());

    let db_result = host_adapter.db_get("http_fetch_result");
    assert!(db_result.is_some());

    let result: PromiseStatus = serde_json::from_str(&db_result.unwrap()).unwrap();
    assert!(matches!(result, PromiseStatus::Fulfilled(_)));

    let result = match result {
        PromiseStatus::Fulfilled(data) => String::from_utf8(data).unwrap(),
        _ => panic!("Promise should be fulfilled"),
    };
    // Compare result with real API fetch
    let expected_result = reqwest::get(fetch_url).await.unwrap().text().await.unwrap();
    assert_eq!(result, expected_result);
}
