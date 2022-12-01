mod promises;

#[cfg(feature = "wasm")]
pub mod wasm;

use std::{str::FromStr, string::ParseError};
use parse_display::{Display, FromStr};

pub use promises::{
    CallSelfAction,
    ChainChangeAction,
    ChainViewAction,
    DatabaseGetAction,
    DatabaseSetAction,
    HttpAction,
    Promise,
    PromiseAction,
    PromiseStatus,
};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, FromStr, Display)]
pub enum Chain {
    Near,
    Cosmos
}


