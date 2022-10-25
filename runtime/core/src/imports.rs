use wasmer::{imports, Array, Function, ImportObject, Module, Store, WasmPtr};
use wasmer_wasi::WasiEnv;

use super::{
    context::VmContext,
    promise::{Promise, PromiseQueueBP},
    AdapterTypes,
    DatabaseAdapter,
};

/// Adds a new promise to the promises stack
pub fn promise_then_import_obj<Adapters: AdapterTypes>(store: &Store, vm_context: VmContext<Adapters>) -> Function {
    fn promise_result_write<Adapters: AdapterTypes>(env: &VmContext<Adapters>, ptr: WasmPtr<u8, Array>, length: i32) {
        let memory_ref = env.memory.get_ref().unwrap();
        let mut adapter_ref = env.adapters.lock().unwrap();
        let mut promises_ref = env.promises.lock().unwrap();

        let promise_data_raw = ptr.get_utf8_string(memory_ref, length as u32).unwrap();
        println!("{}", &promise_data_raw);
        let promise: Promise = serde_json::from_str(&promise_data_raw).unwrap();

        adapter_ref.database.set("key", "somevalue");
        promises_ref.add(promise);
    }

    Function::new_native_with_env(store, vm_context, promise_result_write)
}

/// Gets the length (stringified) of the promise status
pub fn promise_status_length_import_obj<Adapters: AdapterTypes>(
    store: &Store,
    vm_context: VmContext<Adapters>,
) -> Function {
    fn promise_status_length<Adapters: AdapterTypes>(env: &VmContext<Adapters>, promise_index: i32) -> i64 {
        let promises_ref = env.promises.lock().unwrap();
        let promise_info = promises_ref.queue.get(promise_index as usize).unwrap();

        // The length depends on the full status enum + result in JSON
        serde_json::to_string(&promise_info.status)
            .unwrap()
            .len()
            .try_into()
            .unwrap()
    }

    Function::new_native_with_env(store, vm_context, promise_status_length)
}

/// Writes the status of the promise to the WASM memory
pub fn promise_status_write_import_obj<Adapters: AdapterTypes>(
    store: &Store,
    vm_context: VmContext<Adapters>,
) -> Function {
    fn promise_status_write<Adapters: AdapterTypes>(
        env: &VmContext<Adapters>,
        promise_index: i32,
        result_data_ptr: WasmPtr<u8, Array>,
        result_data_length: i64,
    ) {
        let memory_ref = env.memory.get_ref().unwrap();
        let promises_ref = env.promises.lock().unwrap();
        let promise_info = promises_ref.queue.get(promise_index as usize).unwrap();
        let promise_status = serde_json::to_string(&promise_info.status).unwrap();
        let promise_status_bytes = promise_status.as_bytes();

        let derefed_ptr = result_data_ptr.deref(memory_ref, 0, result_data_length as u32).unwrap();

        for index in 0..result_data_length {
            derefed_ptr
                .get(index as usize)
                .unwrap()
                .set(promise_status_bytes[index as usize]);
        }
    }

    Function::new_native_with_env(store, vm_context, promise_status_write)
}

pub fn create_wasm_imports<Adapters: AdapterTypes>(
    store: &Store,
    vm_context: VmContext<Adapters>,
    wasi_env: &mut WasiEnv,
    wasm_module: &Module,
) -> ImportObject {
    let host_import_obj = imports! {
        "env" => {
            "promise_then" => promise_then_import_obj(store, vm_context.clone()),
            "promise_status_length" => promise_status_length_import_obj(store, vm_context.clone()),
            "promise_status_write" => promise_status_write_import_obj(store, vm_context)
        }
    };

    // Combining the WASI exports with our custom (host) imports
    let mut wasi_import_obj = wasi_env.import_object(wasm_module).unwrap();
    let host_exports = host_import_obj.get_namespace_exports("env").unwrap();
    wasi_import_obj.register("env", host_exports);

    wasi_import_obj
}
