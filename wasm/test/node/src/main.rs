use seda_runtime_sdk::{
    wasm::{call_self, shared_memory_get, shared_memory_set},
    FromBytes,
    ToBytes,
};

fn main() {
    println!("Hello World");

    let foo_get = shared_memory_get("foo");
    if !foo_get.is_empty() {
        let bar = String::from_bytes_vec(foo_get).unwrap();
        println!("already set foo: {bar}");
        return;
    }

    shared_memory_set("foo", "bar".to_bytes().eject());
    call_self("shared_memory_success", Vec::new()).start();
}

#[no_mangle]
fn shared_memory_success() {
    let foo_get = shared_memory_get("foo");
    let bar = String::from_bytes_vec(foo_get).unwrap();
    println!("read: {bar}");

    shared_memory_set("done", true.to_bytes().eject());
}
