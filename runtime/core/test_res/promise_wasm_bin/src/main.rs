use std::env;

use crate::promise::{call_self, db_get, db_set, Promise};

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
    println!("Last step completed");

    Promise::result(2);
}
