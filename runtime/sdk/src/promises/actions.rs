use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PromiseAction {
    CallSelf(CallSelfAction),
    DatabaseSet(DatabaseSetAction),
    DatabaseGet(DatabaseGetAction),
    Http(HttpAction),
    ChainView(ChainViewAction),
    ChainCall(ChainCallAction),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CallSelfAction {
    pub function_name: String,
    pub args:          Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseSetAction {
    pub key:   String,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseGetAction {
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpAction {
    pub url: String,
    // TODO: add headers, method, etc :)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChainViewAction {
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChainCallAction {
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub deposit:     String,
}
