use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn build_and_copy(node_dir: &Path, wasm_bin: &str) {
    let wasm_file = format!("{}.wasm", wasm_bin.replace('-', "_"));
    let mut node_dir = node_dir.to_path_buf();

    let mut wasm_bin_path = node_dir.to_path_buf();
    wasm_bin_path.pop();

    let mut wasm_dir = node_dir.to_path_buf();
    wasm_dir.pop();
    wasm_dir.push("wasm");
    wasm_dir.push(wasm_bin);

    dbg!(wasm_dir.display(), wasm_dir.exists());
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

    node_dir.push("wasm_files");
    if !node_dir.exists() {
        fs::create_dir_all(&node_dir).expect("Failed to create directory.");
    }
    node_dir.push(wasm_file);

    dbg!(wasm_bin_path.display(), wasm_bin_path.exists());
    dbg!(node_dir.display(), node_dir.exists());
    fs::copy(wasm_bin_path, node_dir).unwrap_or_else(|e| panic!("Failed to copy {wasm_bin}.wasm: `{e}`"));
}

fn main() {
    // TODO these ones should be test only
    println!("cargo:rerun-if-changed=../../wasm/cli");

    let node_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    build_and_copy(&node_dir, "cli");
}
