use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn build_and_copy(runtime_core_dir: &Path, wasm_bin: &str) {
    let wasm_file = format!("{}.wasm", wasm_bin);
    let mut runtime_core_dir = runtime_core_dir.to_path_buf();

    let mut wasm_bin_path = runtime_core_dir.to_path_buf();
    wasm_bin_path.pop();
    wasm_bin_path.pop();

    let mut wasm_dir = runtime_core_dir.to_path_buf();
    wasm_dir.pop();
    wasm_dir.pop();
    wasm_dir.push("wasm");
    wasm_dir.push("test");
    wasm_dir.push(wasm_bin);

    let status = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasi")
        .arg("--release")
        .current_dir(&wasm_dir)
        .status()
        .expect("failed to execute cargo");
    assert!(status.success(), "failed to build wasm/{wasm_bin}");

    wasm_bin_path.push("target");
    wasm_bin_path.push("wasm32-wasi");
    wasm_bin_path.push("release");
    wasm_bin_path.push(&wasm_file);

    runtime_core_dir.push("test_files");
    if !runtime_core_dir.exists() {
        fs::create_dir_all(&runtime_core_dir).expect("Failed to create directory.");
    }
    runtime_core_dir.push(wasm_file);

    fs::copy(wasm_bin_path, runtime_core_dir).unwrap_or_else(|e| panic!("Failed to copy {wasm_bin}.wasm: `{e}`"));
}

fn main() {
    println!("cargo:rerun-if-changed=../../wasm/test/demo-cli");
    println!("cargo:rerun-if-changed=../../wasm/test/promise-wasm-bin");


    let runtime_core_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    build_and_copy(&runtime_core_dir, "demo-cli");
    build_and_copy(&runtime_core_dir, "promise-wasm-bin");

}


