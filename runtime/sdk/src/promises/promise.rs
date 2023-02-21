use serde::{Deserialize, Serialize};

use super::PromiseAction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PromiseStatus {
    /// Initial state
    Unfulfilled,

    /// We are processing the promise
    Pending,

    /// The promise completed
    Fulfilled(Vec<u8>),

    /// There was an error executing this promise
    Rejected(Vec<u8>),
}

impl<T: crate::ToBytes, E: crate::ToBytes> From<Result<T, E>> for PromiseStatus {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(fulfilled) => PromiseStatus::Fulfilled(fulfilled.to_bytes().eject()),
            Err(rejection) => PromiseStatus::Rejected(rejection.to_bytes().eject()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Promise {
    /// The name of the action we should execute
    pub action: PromiseAction,

    /// The status of the promise, will include the result if it's fulfilled
    pub status: PromiseStatus,
}
