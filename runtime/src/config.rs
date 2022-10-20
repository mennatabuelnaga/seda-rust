use wasmer_wasi::{WasiEnv, WasiState};

#[derive(Clone)]
pub struct VmConfig {
    /// Name of the binary, ex. "consensus", "fisherman", etc
    pub program_name: String,

    // The function we need to execute, defaults to the WASI default ("_start")
    pub start_func: Option<String>,

    /// Arguments to pass to the WASM binary
    pub args: Vec<String>,

    /// The WASM binary as a byte array
    pub wasm_binary: Vec<u8>,

    pub debug: bool,
}

impl VmConfig {
    pub fn finalize(self) -> WasiEnv {
        let mut wasi_state = WasiState::new(&self.program_name);
        wasi_state.args(&self.args);
        wasi_state.finalize().unwrap()
    }
}
