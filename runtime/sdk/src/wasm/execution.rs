use super::{raw, Promise};
use crate::{events::Event, PromiseAction, TriggerEventAction};

pub fn execution_result(result: Vec<u8>) {
    let result_length = result.len() as i32;

    unsafe {
        raw::execution_result(result.as_ptr(), result_length);
    }
}

/// Triggers an event on the host node
/// Allows you to resolve data requests, sign blocks but at a later stage
pub fn trigger_event(event: Event) -> Promise {
    Promise::new(PromiseAction::TriggerEvent(TriggerEventAction { event }))
}
