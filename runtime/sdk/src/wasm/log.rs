use super::raw;
use crate::Level;

pub fn _log(level: Level, msg: &str) {
    let level_str = serde_json::to_string(&level).unwrap();

    // TODO pass file and line to this, but only once I figure out
    // how to overwrite the metadata from tracing.
    unsafe {
        raw::_log(
            level_str.as_ptr(),
            level_str.len() as i32,
            msg.as_ptr(),
            msg.len() as i64,
        );
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
				let _msg = format!($($arg)*);
				#[cfg(debug_assertions)]
				let _msg = format!("{_msg}\n    at {}:{}", file!(), line!());
				seda_runtime_sdk::wasm::_log($level, &_msg)
    };
}

pub use log;
