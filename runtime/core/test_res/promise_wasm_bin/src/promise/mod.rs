use std::str;

use serde::Serialize;
use serde_json::json;

use self::raw::promise_then;

mod raw;

use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub enum PromiseAction {
    CallSelf(CallSelfAction),
    DatabaseSet(DatabaseSetAction),
    DatabaseGet(DatabaseGetAction),
}

#[derive(Serialize, Deserialize)]
pub struct CallSelfAction {
    pub function_name: String,
    pub args:          Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseSetAction {
    pub key:   String,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseGetAction {
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub enum PromiseStatus {
    /// Initial state
    Unfulfilled,

    /// We are processing the promise
    Pending,

    /// The promise completed
    Fulfilled(Vec<u8>),

    /// There was an error executing this promise
    Rejected(Vec<u8>),
}

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

    pub fn result(index: i32) {
        let mut promise_result_length: i64 = 0;

        unsafe {
            promise_result_length = raw::promise_status_length(index);
        }

        let mut result_data: Vec<u8> = Vec::new();
        result_data.resize(promise_result_length as usize, 0);

        unsafe {
            raw::promise_status_write(index, result_data.as_mut_ptr(), promise_result_length);
        }

        let result = str::from_utf8(&result_data).unwrap();
        println!("Promise Result: {}", result);
    }
}

pub fn db_set(key: &str, value: &str) -> Promise {
    Promise::new(PromiseAction::DatabaseSet(DatabaseSetAction {
        key:   key.to_string(),
        value: value.to_string().into_bytes(),
    }))
}

pub fn db_get(key: &str) -> Promise {
    Promise::new(PromiseAction::DatabaseGet(DatabaseGetAction { key: key.to_string() }))
}

pub fn call_self(function_name: &str, args: Vec<String>) -> Promise {
    Promise::new(PromiseAction::CallSelf(CallSelfAction {
        function_name: function_name.to_string(),
        args,
    }))
}
