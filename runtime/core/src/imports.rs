use wasmer::{imports, Array, Function, ImportObject, Memory, Module, Store, WasmPtr};
use wasmer_wasi::WasiEnv;

use super::{MemoryAdapter, Result, RuntimeError, VmContext};

/// Wrapper around memory.get_ref to implement the RuntimeError
fn get_memory(env: &VmContext) -> Result<&Memory> {
    Ok(env.memory.get_ref().ok_or("Memory reference could not be retrieved")?)
}

/// Adds a new promise to the promises stack
pub fn promise_then_import_obj(store: &Store, vm_context: VmContext) -> Function {
    fn promise_result_write(env: &VmContext, ptr: WasmPtr<u8, Array>, length: i32) -> Result<()> {
        let memory_ref = get_memory(env)?;
        let mut promises_queue_ref = env.promise_queue.lock();

        let promise_data_raw = ptr
            .get_utf8_string(memory_ref, length as u32)
            .ok_or("Error getting promise data")?;

        let promise = serde_json::from_str(&promise_data_raw)?;

        promises_queue_ref.add_promise(promise);

        Ok(())
    }

    Function::new_native_with_env(store, vm_context, promise_result_write)
}

/// Gets the length (stringified) of the promise status
pub fn promise_status_length_import_obj(store: &Store, vm_context: VmContext) -> Function {
    fn promise_status_length(env: &VmContext, promise_index: i32) -> Result<i64> {
        let promises_queue_ref = env.current_promise_queue.lock();

        let promise_info = promises_queue_ref
            .queue
            .get(promise_index as usize)
            .ok_or_else(|| format!("Could not find promise at index: {}", promise_index))?;

        // The length depends on the full status enum + result in JSON
        let status = serde_json::to_string(&promise_info.status)?;

        Ok(status.len() as i64)
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
    ) -> Result<()> {
        let memory_ref = get_memory(env)?;
        let promises_ref = env.current_promise_queue.lock();
        let promise_info = promises_ref
            .queue
            .get(promise_index as usize)
            .ok_or_else(|| RuntimeError::VmHostError(format!("Could not find promise at index: {}", promise_index)))?;

        let promise_status = serde_json::to_string(&promise_info.status)?;

        let promise_status_bytes = promise_status.as_bytes();
        let derefed_ptr = result_data_ptr
            .deref(memory_ref, 0, result_data_length as u32)
            .ok_or("Invalid pointer")?;

        for index in 0..result_data_length {
            derefed_ptr
                .get(index as usize)
                .ok_or("Writing out of bounds to memory")?
                .set(promise_status_bytes[index as usize]);
        }

        Ok(())
    }

    Function::new_native_with_env(store, vm_context, promise_status_write)
}

pub fn read_memory(store: &Store, vm_context: VmContext) -> Function {
    fn read_from_memory(
        env: &VmContext,
        key: WasmPtr<u8, Array>,
        key_len: i32,
        result_data_ptr: WasmPtr<u8, Array>,
        result_data_length: i64,
    ) -> Result<()> {
        let memory_ref = get_memory(env)?;
        let key = key
            .get_utf8_string(memory_ref, key_len as u32)
            .ok_or("Error getting promise data")?;

        let memory_adapter = env.memory_adapter.lock();
        let read_value: Vec<u8> = memory_adapter.get(&key)?.unwrap_or_default();
        if result_data_length as usize != read_value.len() {
            todo!("throw error")
        }

        let derefed_ptr = result_data_ptr
            .deref(memory_ref, 0, result_data_length as u32)
            .ok_or("Invalid pointer")?;
        for (index, byte) in read_value.iter().enumerate().take(result_data_length as usize) {
            derefed_ptr
                .get(index)
                .ok_or("Writing out of bounds to memory")?
                .set(*byte);
        }
        Ok(())
    }

    Function::new_native_with_env(store, vm_context, read_from_memory)
}

pub fn write_memory(store: &Store, vm_context: VmContext) -> Function {
    fn write_to_memory(
        env: &VmContext,
        key: WasmPtr<u8, Array>,
        key_len: i32,
        value: WasmPtr<u8, Array>,
        value_len: i32,
    ) -> Result<()> {
        let memory_ref = get_memory(env)?;
        let key = key
            .get_utf8_string(memory_ref, key_len as u32)
            .ok_or("Error getting promise data")?;
        let value = value.deref(memory_ref, 0, value_len as u32).ok_or("Invalid pointer")?;
        let value_bytes: Vec<u8> = value.into_iter().map(|wc| wc.get()).collect();

        let mut memory_adapter = env.memory_adapter.lock();
        memory_adapter.put(&key, value_bytes);

        Ok(())
    }

    Function::new_native_with_env(store, vm_context, write_to_memory)
}

pub fn create_wasm_imports(
    store: &Store,
    vm_context: VmContext,
    wasi_env: &mut WasiEnv,
    wasm_module: &Module,
) -> Result<ImportObject> {
    let host_import_obj = imports! {
        "env" => {
            "promise_then" => promise_then_import_obj(store, vm_context.clone()),
            "promise_status_length" => promise_status_length_import_obj(store, vm_context.clone()),
            "promise_status_write" => promise_status_write_import_obj(store, vm_context.clone()),
            "read_memory" => read_memory(store, vm_context.clone()),
            "write_memory" => write_memory(store, vm_context)
        }
    };

    // Combining the WASI exports with our custom (host) imports
    let mut wasi_import_obj = wasi_env.import_object(wasm_module)?;
    let host_exports = host_import_obj
        .get_namespace_exports("env")
        .ok_or("VM could not get env namespace")?;
    wasi_import_obj.register("env", host_exports);

    Ok(wasi_import_obj)
}
