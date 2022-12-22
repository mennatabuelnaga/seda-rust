use super::raw;

pub fn config_result(result: Vec<u8>) {
    let result_length = result.len() as i32;

    unsafe {
        raw::config_result(result.as_ptr(), result_length);
    }
}
