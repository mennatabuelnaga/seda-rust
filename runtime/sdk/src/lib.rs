mod promises;

#[cfg(feature = "wasm")]
pub mod wasm;

use parse_display::{Display, FromStr};
pub use promises::{
    CallSelfAction,
    ChainCallAction,
    ChainViewAction,
    DatabaseGetAction,
    DatabaseSetAction,
    HttpAction,
    Promise,
    PromiseAction,
    PromiseStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, FromStr, Display)]
pub enum Chain {
    Near,
    Cosmos,
}
