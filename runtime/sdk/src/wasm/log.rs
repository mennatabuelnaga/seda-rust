use serde::{Deserialize, Serialize};

use super::raw;

#[derive(Serialize, Deserialize)]
pub enum Level {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
}

pub fn log(level: Level, msg: &str) {
    let level_str = serde_json::to_string(&level).unwrap();

    unsafe {
        raw::_log(
            level_str.as_ptr(),
            level_str.len() as i32,
            msg.as_ptr(),
            msg.len() as i64,
        );
    }
}
