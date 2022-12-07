use super::Promise;
use crate::{CallSelfAction, PromiseAction};

pub fn call_self(function_name: &str, args: Vec<String>) -> Promise {
    println!("call_self");
    dbg!(Promise::new(PromiseAction::CallSelf(CallSelfAction {
        function_name: function_name.to_string(),
        args,
    })))
}
