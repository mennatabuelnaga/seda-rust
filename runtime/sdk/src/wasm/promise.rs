use std::str;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::raw::promise_then;
use crate::{wasm::raw, PromiseAction, PromiseStatus};

#[derive(Serialize, Deserialize)]
pub struct Promise {
    /// The name of the action we should execute
    pub action: PromiseAction,

    /// The status of the promise, will include the result if it's fulfilled
    pub status: PromiseStatus,

    /// The promise we should execute after this one
    pub after: Option<Box<Self>>,
}

impl Promise {
    pub fn new(action: PromiseAction) -> Self {
        Self {
            action,
            after: None,
            status: PromiseStatus::Unfulfilled,
        }
    }

    fn add_to_queue(promise: &Self) {
        let promise_data = json!({
            "action": promise.action,
            "status": promise.status,
        })
        .to_string();

        unsafe {
            promise_then(promise_data.as_ptr(), promise_data.len() as i32);
        }
    }

    /// Starts the promise chain, must be only called on the first promise
    /// before chaining (calling .then())
    pub fn start(self) -> Self {
        Promise::add_to_queue(&self);

        self
    }

    /// Chains this promise after the previous promise
    pub fn then(mut self, after: Self) -> Self {
        Promise::add_to_queue(&after);
        self.after = Some(Box::new(after));

        self
    }

    pub fn result(index: i32) -> Vec<u8> {
        let promise_result_length = unsafe { raw::promise_status_length(index) };

        let mut result_data: Vec<u8> = Vec::new();
        result_data.resize(promise_result_length as usize, 0);

        unsafe {
            raw::promise_status_write(index, result_data.as_mut_ptr(), promise_result_length);
        }

        let result = str::from_utf8(&result_data).unwrap();
        println!("Promise Result: {}", result);

        result_data
    }
}
