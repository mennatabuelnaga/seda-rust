extern "C" {
    pub fn promise_then(action_data_offset: *const u8, action_data_length: i32);
    pub fn promise_status_length(promise_index: i32) -> i64;
    pub fn promise_status_write(promise_index: i32, result_data_offset: *const u8, result_data_length: i64);
    pub fn read_memory(key: *const u8, key_length: i64, result_data_ptr: *const u8, result_data_length: i64);
    pub fn write_memory(key: *const u8, key_length: i64, value: *const u8, value_length: i64);
}
