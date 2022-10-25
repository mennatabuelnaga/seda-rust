use std::sync::{Arc, Mutex};

use wasmer::{Instance, Module, Store};

use super::{imports::create_wasm_imports, AdapterTypes, Adapters, VmConfig, VmContext};
use crate::adapters::DatabaseAdapter;

pub struct VmResult {}

pub fn start_runtime<Types: AdapterTypes>(
    config: VmConfig,
    adapters: Arc<Mutex<Adapters<Types>>>,
) -> Result<VmResult, String> {
    let wasm_store = Store::default();

    // TODO: Good pratices on how to handle errors
    let wasm_module = Module::new(&wasm_store, &config.wasm_binary).unwrap();
    let func_name = config.clone().start_func.unwrap_or_else(|| "_start".to_string());
    let mut wasi_env = config.finalize();
    let vm_context = VmContext::create_vm_context(adapters);

    let imports = create_wasm_imports(&wasm_store, vm_context.clone(), &mut wasi_env, &wasm_module);
    let wasmer_instance = Instance::new(&wasm_module, &imports).unwrap();
    let main_func = wasmer_instance.exports.get_function(&func_name).unwrap();

    main_func.call(&[]).unwrap();

    // Checking values here
    let adapter_ref = vm_context.adapters.lock().unwrap();
    adapter_ref.database.get("key");

    Result::Ok(VmResult {})
}
