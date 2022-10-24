use std::env;

use crate::promise::{call_self, db_query, Promise};

mod promise;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Hello World {:?}", args);

    db_query().start().then(call_self(
        "db_fetch_success".to_string(),
        "Somedata".to_string().into_bytes(),
    ));

    Promise::result(0);
}
