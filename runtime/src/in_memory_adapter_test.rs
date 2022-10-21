use super::{InMemory, MemoryAdapter};

#[test]
fn test_in_memory_storage() {
    let mut memory_adapter = InMemory::default();

    let input_1 = 245u32;
    memory_adapter.write("u32", input_1).expect("No error");

    let input_2 = "hello".to_string();
    memory_adapter.write("string", input_2.clone()).expect("No error");

    assert_eq!(memory_adapter.read("u32").unwrap(), Some(input_1));
    assert_eq!(memory_adapter.read("string").unwrap(), Some(input_2));
}

#[test]
fn test_in_memory_storage_duplicate_key() {
    let mut memory_adapter = InMemory::default();

    memory_adapter.write("u32", 245u32).expect("No error");
    assert!(memory_adapter.write("u32", 2u32).is_err());
}
