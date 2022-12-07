use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PromiseAction {
    CallSelf(CallSelfAction),
    DatabaseSet(DatabaseSetAction),
    DatabaseGet(DatabaseGetAction),

    Http(HttpAction),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallSelfAction {
    pub function_name: String,
    pub args:          Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseSetAction {
    pub key:   String,
    pub value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseGetAction {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpAction {
    pub url: String,
    // TODO: add headers, method, etc :)
}
