use wasmer::{imports, Array, Function, ImportObject, Memory, Module, Store, WasmPtr};
use wasmer_wasi::WasiEnv;

use super::{Promise, VmContext};
use crate::RuntimeError;

/// Wrapper around memory.get_ref to implement the RuntimeError
fn get_memory(env: &VmContext) -> Result<&Memory, RuntimeError> {
    match env.memory.get_ref() {
        Some(memory) => Ok(memory),
        None => Err(RuntimeError::VmHostError(
            "Memory reference could not be retrieved".to_string(),
        )),
    }
}

/// Adds a new promise to the promises stack
pub fn promise_then_import_obj(store: &Store, vm_context: VmContext) -> Function {
    fn promise_result_write(env: &VmContext, ptr: WasmPtr<u8, Array>, length: i32) -> Result<(), RuntimeError> {
        let memory_ref = get_memory(env)?;
        let mut promises_queue_ref = env.promise_queue.lock();

        let promise_data_raw = match ptr.get_utf8_string(memory_ref, length as u32) {
            Some(data) => data,
            None => return Err(RuntimeError::VmHostError("Error getting promise data".to_string())),
        };

        let promise: Promise = match serde_json::from_str(&promise_data_raw) {
            Ok(prom) => prom,
            Err(err) => return Err(RuntimeError::VmHostError(err.to_string())),
        };

        promises_queue_ref.add_promise(promise);

        Ok(())
    }

    Function::new_native_with_env(store, vm_context, promise_result_write)
}

/// Gets the length (stringified) of the promise status
pub fn promise_status_length_import_obj(store: &Store, vm_context: VmContext) -> Function {
    fn promise_status_length(env: &VmContext, promise_index: i32) -> Result<i64, RuntimeError> {
        let promises_queue_ref = env.current_promise_queue.lock();

        let promise_info = match promises_queue_ref.queue.get(promise_index as usize) {
            Some(info) => info,
            None => {
                return Err(RuntimeError::VmHostError(format!(
                    "Could not find promise at index {}",
                    promise_index
                )));
            }
        };

        // The length depends on the full status enum + result in JSON
        let length = match serde_json::to_string(&promise_info.status) {
            Ok(result) => result.len(),
            Err(err) => return Err(RuntimeError::VmHostError(err.to_string())),
        };

        Ok(length as i64)
    }

    Function::new_native_with_env(store, vm_context, promise_status_length)
}

/// Writes the status of the promise to the WASM memory
pub fn promise_status_write_import_obj(store: &Store, vm_context: VmContext) -> Function {
    fn promise_status_write(
        env: &VmContext,
        promise_index: i32,
        result_data_ptr: WasmPtr<u8, Array>,
        result_data_length: i64,
    ) -> Result<(), RuntimeError> {
        let memory_ref = get_memory(env)?;
        let promises_ref = env.current_promise_queue.lock();
        let promise_info = match promises_ref.queue.get(promise_index as usize) {
            Some(value) => value,
            None => {
                return Err(RuntimeError::VmHostError(format!(
                    "No promise found at index: {}",
                    promise_index
                )));
            }
        };

        let promise_status = match serde_json::to_string(&promise_info.status) {
            Ok(value) => value,
            Err(error) => return Err(RuntimeError::VmHostError(error.to_string())),
        };

        let promise_status_bytes = promise_status.as_bytes();
        let derefed_ptr = match result_data_ptr.deref(memory_ref, 0, result_data_length as u32) {
            Some(value) => value,
            None => return Err(RuntimeError::VmHostError("Invalid pointer".to_string())),
        };

        for index in 0..result_data_length {
            match derefed_ptr.get(index as usize) {
                Some(value) => value.set(promise_status_bytes[index as usize]),
                None => return Err(RuntimeError::VmHostError("Writing out of bounds to memory".to_string())),
            };
        }

        Ok(())
    }

    Function::new_native_with_env(store, vm_context, promise_status_write)
}

pub fn create_wasm_imports(
    store: &Store,
    vm_context: VmContext,
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
