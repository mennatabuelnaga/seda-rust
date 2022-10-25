use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum PromiseAction {
    CallSelf(CallSelfAction),
    DatabaseSet(DatabaseSetAction),
    DatabaseGet(DatabaseGetAction),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallSelfAction {
    pub function_name: String,
    pub args:          Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseSetAction {
    pub key:   String,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseGetAction {
    pub key: String,
}
