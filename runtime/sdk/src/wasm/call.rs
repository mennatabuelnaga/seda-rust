use super::Promise;
use crate::{CallSelfAction, PromiseAction};

pub fn call_self(function_name: &str, args: Vec<String>) -> Promise {
    Promise::new(PromiseAction::CallSelf(CallSelfAction {
        function_name: function_name.to_string(),
        args,
    }))
}

#[macro_export]
macro_rules! call_self {
    (@count $count:expr, $args:expr, $body:block) => {
			let fn_name = format!("main_step_{}", $count);
			call_self(&fn_name, args);

		};
    (args: [$($arg:expr),*], $body:block) => {
			let args = vec![$($arg),*];
			call_self!(@count 0, args, $body)
		};
}
