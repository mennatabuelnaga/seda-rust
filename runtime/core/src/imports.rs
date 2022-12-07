use seda_runtime_adapters::MemoryAdapter;
use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut, Imports, Module, Store, WasmPtr};
use wasmer_wasi::WasiFunctionEnv;

use super::{Result, RuntimeError, VmContext};

/// Adds a new promise to the promises stack
pub fn promise_then_import_obj(store: &mut Store, vm_context: &FunctionEnv<VmContext>) -> Function {
    fn promise_result_write(env: FunctionEnvMut<'_, VmContext>, ptr: WasmPtr<u8>, length: i32) -> Result<()> {
        let ctx = env.data();
        let memory = ctx.memory_view(&env);
        let mut promises_queue_ref = ctx.promise_queue.lock();

        let promise_data_raw = ptr.read_utf8_string(&memory, length as u32)?;

        let promise = serde_json::from_str(&promise_data_raw)?;
        promises_queue_ref.add_promise(promise);

        Ok(())
    }

    Function::new_typed_with_env(store, vm_context, promise_result_write)
}

/// Gets the length (stringified) of the promise status
pub fn promise_status_length_import_obj(store: &mut Store, vm_context: &FunctionEnv<VmContext>) -> Function {
    fn promise_status_length(env: FunctionEnvMut<'_, VmContext>, promise_index: i32) -> Result<i64> {
        let ctx = env.data();
        let promises_queue_ref = ctx.current_promise_queue.lock();

        let promise_info = promises_queue_ref
            .queue
            .get(promise_index as usize)
            .ok_or_else(|| format!("Could not find promise at index: {}", promise_index))?;

        dbg!(&promise_info);
        // The length depends on the full status enum + result in JSON
        let status = serde_json::to_string(&promise_info.status)?;

        Ok(status.len() as i64)
    }

    Function::new_typed_with_env(store, vm_context, promise_status_length)
}

/// Writes the status of the promise to the WASM memory
pub fn promise_status_write_import_obj(store: &mut Store, vm_context: &FunctionEnv<VmContext>) -> Function {
    fn promise_status_write(
        env: FunctionEnvMut<'_, VmContext>,
        promise_index: i32,
        result_data_ptr: WasmPtr<u8>,
        result_data_length: i64,
    ) -> Result<()> {
        let ctx = env.data();
        let memory = ctx.memory_view(&env);
        let promises_ref = ctx.current_promise_queue.lock();
        let promise_info = promises_ref
            .queue
            .get(promise_index as usize)
            .ok_or_else(|| RuntimeError::VmHostError(format!("Could not find promise at index: {}", promise_index)))?;

        let promise_status = serde_json::to_string(&promise_info.status)?;
        let promise_status_bytes = promise_status.as_bytes();

        let values = result_data_ptr.slice(&memory, result_data_length as u32)?;

        for index in 0..result_data_length {
            values.index(index as u64).write(promise_status_bytes[index as usize])?;
        }

        Ok(())
    }

    Function::new_typed_with_env(store, vm_context, promise_status_write)
}

/// Reads the value from memory as byte array to the wasm result pointer.
pub fn memory_read_import_obj(store: &mut Store, vm_context: &FunctionEnv<VmContext>) -> Function {
    fn memory_read(
        env: FunctionEnvMut<'_, VmContext>,
        key: WasmPtr<u8>,
        key_length: i64,
        result_data_ptr: WasmPtr<u8>,
        result_data_length: i64,
    ) -> Result<()> {
        let ctx = env.data();
        let memory = ctx.memory_view(&env);

        let key = key.read_utf8_string(&memory, key_length as u32)?;
        let memory_adapter = ctx.memory_adapter.lock();
        let read_value: Vec<u8> = memory_adapter.get(&key)?.unwrap_or_default();

        if result_data_length as usize != read_value.len() {
            Err(format!(
                "The result data length `{result_data_length}` is not the same length for the value `{}`",
                read_value.len()
            ))?;
        }

        let values = result_data_ptr.slice(&memory, result_data_length as u32)?;

        for index in 0..result_data_length {
            values.index(index as u64).write(read_value[index as usize])?;
        }

        Ok(())
    }

    Function::new_typed_with_env(store, vm_context, memory_read)
}

/// Reads the value from memory as byte array and sends the number of bytes to
/// WASM.
pub fn memory_read_length_import_obj(store: &mut Store, vm_context: &FunctionEnv<VmContext>) -> Function {
    fn memory_read_length(env: FunctionEnvMut<'_, VmContext>, key: WasmPtr<u8>, key_length: i64) -> Result<i64> {
        let ctx = env.data();
        let memory = ctx.memory_view(&env);

        let key = key.read_utf8_string(&memory, key_length as u32)?;

        let memory_adapter = ctx.memory_adapter.lock();
        let read_value: Vec<u8> = memory_adapter.get(&key)?.unwrap_or_default();

        Ok(read_value.len() as i64)
    }

    Function::new_typed_with_env(store, vm_context, memory_read_length)
}

/// Writes the value from WASM to the memory storage object.
pub fn memory_write_import_obj(store: &mut Store, vm_context: &FunctionEnv<VmContext>) -> Function {
    fn memory_write(
        _env: FunctionEnvMut<'_, VmContext>,
        _key: WasmPtr<u8>,
        _key_length: i64,
        _value: WasmPtr<u8>,
        _value_len: i64,
    ) -> Result<()> {
        // let ctx = env.data();
        // let memory = ctx.memory_view(&env);
        // let key = key.read_utf8_string(&memory, key_length as u32)?;
        // // let value = value.read(&memory)?;
        // let mut value_bytes = Vec::new();

        // memory.read(value.offset().into(), value_bytes);

        // let value = value.deref(memory_ref, 0, value_len as u32).ok_or("Invalid
        // pointer")?; let value_bytes: Vec<u8> = value.into_iter().map(|wc|
        // wc.get()).collect();

        // let mut memory_adapter = env.memory_adapter.lock();
        // memory_adapter.put(&key, value_bytes);

        Ok(())
    }

    Function::new_typed_with_env(store, vm_context, memory_write)
}

// Creates the WASM function imports with the stringed names.
pub fn create_wasm_imports(
    store: &mut Store,
    vm_context: FunctionEnv<VmContext>,
    wasi_env: &mut WasiFunctionEnv,
    wasm_module: &Module,
) -> Result<Imports> {
    let host_import_obj = imports! {
        "env" => {
            "promise_then" => promise_then_import_obj(store, &vm_context),
            "promise_status_length" => promise_status_length_import_obj(store,
    &vm_context),         "promise_status_write" =>
    promise_status_write_import_obj(store, &vm_context),
            "memory_read" => memory_read_import_obj(store, &vm_context),
            "memory_read_length" => memory_read_length_import_obj(store,
    &vm_context),         "memory_write" => memory_write_import_obj(store,
    &vm_context)     }
    };

    // Combining the WASI exports with our custom (host) imports
    let mut wasi_import_obj = wasi_env.import_object(store, wasm_module)?;
    let host_exports = host_import_obj
        .get_namespace_exports("env")
        .ok_or("VM could not get env namespace")?;

    wasi_import_obj.register_namespace("env", host_exports);

    Ok(wasi_import_obj)
}
