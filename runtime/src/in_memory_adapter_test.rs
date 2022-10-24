use super::{InMemory, MemoryAdapter};

#[test]
fn test_in_memory_storage() {
    let mut memory_adapter = InMemory::default();

    let input_1 = 245u32;
    memory_adapter.put("u32", input_1).expect("No error");

    let input_2 = "hello".to_string();
    memory_adapter.put("string", input_2.clone()).expect("No error");

    assert_eq!(memory_adapter.get("u32").unwrap(), Some(input_1));
    assert_eq!(memory_adapter.get("string").unwrap(), Some(input_2));
}

#[test]
fn test_in_memory_storage_overwite_key() {
    let mut memory_adapter = InMemory::default();

    let og = 245u32;
    memory_adapter.put("tbr", og).expect("No error");
    assert_eq!(memory_adapter.get("tbr").unwrap(), Some(og));

    let replacement = "replaced".to_string();
    memory_adapter.put("tbr", replacement.clone()).expect("No error");
    assert_eq!(memory_adapter.get("tbr").unwrap(), Some(replacement));
}
