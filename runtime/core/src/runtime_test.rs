use std::{fs, path::PathBuf, sync::Arc};

use parking_lot::Mutex;
use seda_runtime_sdk::PromiseStatus;

use super::{HostAdapters, InMemory, MemoryAdapter, RunnableRuntime, Runtime, TestAdapters, VmConfig};

fn read_wasm() -> Vec<u8> {
    let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_prefix.push("./test_files/promise-wasm-bin.wasm");

    fs::read(path_prefix).unwrap()
}

fn memory_adapter() -> Arc<Mutex<InMemory>> {
    Arc::new(Mutex::new(InMemory::default()))
}

#[tokio::test]
async fn test_promise_queue_multiple_calls_with_external_traits() {
    let wasm_binary = read_wasm();
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime.start_runtime(
        VmConfig {
            args: vec!["hello world".to_string()],
            program_name: "consensus".to_string(),
            start_func: None,
            wasm_binary,
            debug: true,
        },
        memory_adapter(),
        host_adapter.clone(),
    );

    let vm_result = runtime_execution_result.await;
    assert!(vm_result.is_ok());

    let value = host_adapter.db_get("test_value");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "completed");
}

#[tokio::test]
#[should_panic(expected = "input bytes aren't valid utf-8")]
async fn test_bad_wasm_file() {
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
            memory_adapter(),
            host_adapter.clone(),
        )
        .await;

    assert!(runtime_execution_result.is_err());
    runtime_execution_result.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Missing export non_existing_function")]
async fn test_non_existing_function() {
    let wasm_binary = read_wasm();
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
            memory_adapter(),
            host_adapter.clone(),
        )
        .await;

    assert!(runtime_execution_result.is_err());
    runtime_execution_result.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_promise_queue_http_fetch() {
    let fetch_url = "https://swapi.dev/api/people/2/".to_string();

    let wasm_binary = read_wasm();
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args: vec![fetch_url.clone()],
                program_name: "consensus".to_string(),
                start_func: Some("http_fetch_test".to_string()),
                wasm_binary,
                debug: true,
            },
            memory_adapter(),
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

    println!("Decoded result {}", result);
    assert_eq!(result, expected_result);
}

#[tokio::test]
async fn test_memory_adapter() {
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};
    let memory_adapter = memory_adapter();
    let wasm_binary = read_wasm();

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args: vec!["memory adapter".to_string()],
                program_name: "consensus".to_string(),
                start_func: Some("memory_adapter_test_success".to_string()),
                wasm_binary,
                debug: true,
            },
            memory_adapter.clone(),
            host_adapter.clone(),
        )
        .await;

    assert!(runtime_execution_result.is_ok());

    let memory_adapter_ref = memory_adapter.lock();
    let read_value: Result<Option<Vec<u8>>, _> = memory_adapter_ref.get("u8");
    let expected = 234u8.to_le_bytes().to_vec();
    let expected_str = format!("{expected:?}");
    assert!(read_value.is_ok());
    assert_eq!(read_value.unwrap(), Some(expected));

    let u8_value = host_adapter.db_get("u8_result");
    assert!(u8_value.is_some());
    assert_eq!(u8_value.unwrap(), expected_str);

    let u32_value = host_adapter.db_get("u32_result");
    let expected = 3467u32.to_le_bytes().to_vec();
    let expected_str = format!("{expected:?}");
    assert!(u32_value.is_some());
    assert_eq!(u32_value.unwrap(), expected_str);
}

fn cli_wasm() -> Vec<u8> {
    let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_prefix.push("./test_files/demo-cli.wasm");

    fs::read(path_prefix).unwrap()
}

#[tokio::test]
async fn test_cli_demo() {
    let wasm_binary = cli_wasm();
    let host_adapter = HostAdapters::<TestAdapters>::default();
    let runtime = Runtime {};

    let runtime_execution_result = runtime.start_runtime(
        VmConfig {
            args: vec!["--help".to_string()],
            program_name: "consensus".to_string(),
            start_func: None,
            wasm_binary,
            debug: true,
        },
        memory_adapter(),
        host_adapter.clone(),
    );

    let vm_result = runtime_execution_result.await;
    assert!(vm_result.is_ok());

    let value = host_adapter.db_get("test_value");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "completed");
}
