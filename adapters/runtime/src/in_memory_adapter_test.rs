use super::{InMemory, MemoryAdapter, ToBytes};

#[test]
fn test_in_memory_storage() {
    let mut memory_adapter = InMemory::default();

    let input_1 = 245u32;
    assert!(memory_adapter.put("u32", input_1).is_none());

    let input_2 = "hello".to_string();
    assert!(memory_adapter.put("string", input_2.clone()).is_none());

    assert_eq!(memory_adapter.get("u32").unwrap(), Some(input_1));
    assert_eq!(memory_adapter.get("string").unwrap(), Some(input_2));
}

#[test]
fn test_in_memory_storage_overwite_key() {
    let mut memory_adapter = InMemory::default();

    let og = 245u32;
    memory_adapter.put("tbr", og);
    assert_eq!(memory_adapter.get("tbr").unwrap(), Some(og));

    let replacement = "replaced".to_string();
    assert_eq!(memory_adapter.put("tbr", replacement.clone()), Some(og.to_bytes()));
    assert_eq!(memory_adapter.get("tbr").unwrap(), Some(replacement));
}

#[test]
#[should_panic]
fn test_in_memory_storage_incorrect_read_type() {
    let mut memory_adapter = InMemory::default();

    memory_adapter.put("u32", 245u32);
    let _: Option<u8> = memory_adapter.get("u32").unwrap();
}
