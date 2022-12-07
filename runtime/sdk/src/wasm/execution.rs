use super::raw;

pub fn execution_result(result: Vec<u8>) {
    let result_length = result.len() as i32;

    unsafe {
        raw::execution_result(result.as_ptr(), result_length);
    }
}
