use std::env;

use crate::promise::{call_self, db_get, db_set, http_fetch, read_memory, write_memory, Promise};

mod promise;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Hello World {:?}", args);

    db_set("from_wasm", "somevalue")
        .start()
        .then(db_get("from_wasm"))
        .then(call_self("db_fetch_success", vec!["ArgFromInsideWasm".to_string()]));
}

#[no_mangle]
fn db_fetch_success() {
    let args: Vec<String> = env::args().collect();
    println!("Inside the callback {:?}", args);
    Promise::result(1);

    db_set("another_one", "a")
        .start()
        .then(db_set("x", "y"))
        .then(db_get("another_one"))
        .then(call_self("completed_all", vec![]));
}

#[no_mangle]
fn completed_all() {
    db_set("test_value", "completed").start();

    Promise::result(2);
}

#[no_mangle]
fn http_fetch_test() {
    let args: Vec<String> = env::args().collect();
    println!("Hello world {:?}", args);

    http_fetch(args.get(1).unwrap())
        .start()
        .then(call_self("http_fetch_test_success", vec![]));
}

#[no_mangle]
fn http_fetch_test_success() {
    let result = Promise::result(0);
    let value_to_store = String::from_utf8(result).unwrap();

    db_set("http_fetch_result", &value_to_store).start();
}

#[no_mangle]
fn memory_adapter_test_success() {
    let key = "u8";
    let value = 234u8.to_le_bytes().to_vec();
    write_memory(key, value.clone());

    let read_value = read_memory(key);
    println!("read_value: {read_value:?}");
    assert_eq!(read_value, value);

    let key = "u32";
    let value = 3467u32.to_le_bytes().to_vec();
    write_memory(key, value);
    call_self("memory_adapter_callback_test_success", Vec::new()).start();
}

#[no_mangle]
fn memory_adapter_callback_test_success() {
    let read_value = read_memory("u8");
    db_set("u8_result", &format!("{read_value:?}")).start();
    let read_value = read_memory("u32");
    db_set("u32_result", &format!("{read_value:?}")).start();
}
