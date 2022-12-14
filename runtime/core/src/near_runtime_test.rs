use std::{env, fs, path::PathBuf, sync::Arc};

use parking_lot::Mutex;
use seda_runtime_adapters::{test_host::RuntimeTestAdapter, HostAdapter, InMemory};
use serde_json::json;

use crate::{RunnableRuntime, Runtime, VmConfig};

fn cli_wasm() -> Vec<u8> {
    let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_prefix.push("./test_files/demo-cli.wasm");

    fs::read(path_prefix).unwrap()
}

fn set_env_vars() {
    env::set_var("SEDA_CONFIG_PATH", "../../template_config.toml");
}

fn memory_adapter() -> Arc<Mutex<InMemory>> {
    Arc::new(Mutex::new(InMemory::default()))
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cli_demo_view_chain() {
    set_env_vars();
    let wasm_binary = cli_wasm();

    let mut runtime = Runtime::<RuntimeTestAdapter>::new().await;
    let memory_adapter = memory_adapter();
    runtime.init(wasm_binary).unwrap();
    let contract_id = "mc.mennat0.testnet".to_string();
    let method_name = "get_node_socket_address".to_string();
    let args = json!({"node_id": "12".to_string()}).to_string();

    let runtime_execution_result = runtime
        .start_runtime(
            VmConfig {
                args:         vec!["view".to_string(), contract_id, method_name, args],
                program_name: "consensus".to_string(),
                start_func:   None,
                debug:        true,
            },
            memory_adapter.clone(),
        )
        .await;
    dbg!(&runtime_execution_result);
    assert!(runtime_execution_result.is_ok());

    let db_result = runtime.host_adapter.db_get("chain_view_result").await.unwrap();
    assert!(db_result.is_some());

    assert_eq!(db_result.unwrap(), "127.0.0.1:9000".to_string());
}
