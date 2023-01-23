use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{events::Event, Chain};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PromiseAction {
    CallSelf(CallSelfAction),
    DatabaseSet(DatabaseSetAction),
    DatabaseGet(DatabaseGetAction),
    Http(HttpAction),
    ChainView(ChainViewAction),
    ChainCall(ChainCallAction),
    TriggerEvent(TriggerEventAction),
}

impl PromiseAction {
    #[cfg(not(target_family = "wasm"))]
    pub fn is_limited_action(&self) -> bool {
        matches!(
            self,
            Self::DatabaseGet(_) | Self::DatabaseSet(_) | Self::ChainCall(_) | Self::ChainView(_) | Self::TriggerEvent(_)
        )
    }
}

impl fmt::Display for PromiseAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CallSelf(_) => write!(f, "call_self"),
            Self::DatabaseSet(_) => write!(f, "db_set"),
            Self::DatabaseGet(_) => write!(f, "db_get"),
            Self::Http(_) => write!(f, "http"),
            Self::ChainView(_) => write!(f, "chain_view"),
            Self::ChainCall(_) => write!(f, "chain_call"),
            Self::TriggerEvent(_) => write!(f, "trigger_event"),
        }
    }
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
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChainCallAction {
    pub chain:       Chain,
    pub contract_id: String,
    pub method_name: String,
    pub args:        Vec<u8>,
    pub deposit:     String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TriggerEventAction {
    pub event: Event,
}
