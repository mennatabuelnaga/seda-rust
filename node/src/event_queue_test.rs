use crate::event_queue::{Event, EventData, EventQueue};

#[test]
fn add_item_to_event_queue() {
    let mut queue = EventQueue::default();

    queue.add(Event {
        id:   "test-id".to_string(),
        data: EventData::ChainTick,
    });

    let item = queue.get_next(&[]).unwrap();

    assert_eq!(item.id, "test-id".to_string());
}

#[test]
fn get_item_with_skip() {
    let mut queue = EventQueue::default();

    queue.add(Event {
        id:   "test-id".to_string(),
        data: EventData::ChainTick,
    });

    queue.add(Event {
        id:   "test-id-2".to_string(),
        data: EventData::ChainTick,
    });

    let item = queue.get_next(&["test-id".to_string()]).unwrap();

    assert_eq!(item.id, "test-id-2".to_string());
}

#[test]
fn get_item_should_empty_queue() {
    let mut queue = EventQueue::default();

    queue.add(Event {
        id:   "test-id".to_string(),
        data: EventData::ChainTick,
    });

    queue.add(Event {
        id:   "test-id-2".to_string(),
        data: EventData::ChainTick,
    });

    let item = queue.get_next(&[]).unwrap();
    let item2 = queue.get_next(&[]).unwrap();
    let item3 = queue.get_next(&[]);

    assert_eq!(item.id, "test-id".to_string());
    assert_eq!(item2.id, "test-id-2".to_string());
    assert!(item3.is_none());
}

#[test]
fn get_item_should_empty_queue_with_skip() {
    let mut queue = EventQueue::default();

    queue.add(Event {
        id:   "test-id".to_string(),
        data: EventData::ChainTick,
    });

    queue.add(Event {
        id:   "test-id-2".to_string(),
        data: EventData::ChainTick,
    });

    let item = queue.get_next(&["test-id".to_string()]).unwrap();
    let item2 = queue.get_next(&["test-id".to_string()]);

    assert_eq!(item.id, "test-id-2".to_string());
    assert!(item2.is_none());
}
