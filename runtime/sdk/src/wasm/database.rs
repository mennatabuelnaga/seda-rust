use super::Promise;
use crate::{DatabaseGetAction, DatabaseSetAction, PromiseAction};

pub fn db_set(key: &str, value: &str) -> Promise {
    Promise::new(PromiseAction::DatabaseSet(DatabaseSetAction {
        key:   key.to_string(),
        value: value.to_string().into_bytes(),
    }))
}

pub fn db_get(key: &str) -> Promise {
    Promise::new(PromiseAction::DatabaseGet(DatabaseGetAction { key: key.to_string() }))
}
