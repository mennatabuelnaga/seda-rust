use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PromiseStatus {
    /// Initial state
    Unfullfilled,

    /// We are processing the promise
    Pending,

    /// The promise completed
    Fulfilled(Vec<u8>),

    /// There was an error executing this promise
    Rejected(Vec<u8>),
}

#[derive(Serialize, Deserialize)]
pub struct Promise {
    /// The name of the action we should execute
    pub action_name: String,

    /// A byte array containing the payload which should be passed to the action
    pub payload: Vec<u8>,

    /// The status of the promise, will include the result if it's fulfilled
    pub status: PromiseStatus,
}

#[automock]
pub trait PromiseQueueBP {
    fn new() -> Self;
    fn add(&mut self, promise: Promise);
}

pub struct PromiseQueue {
    pub queue: Vec<Promise>,
}

impl PromiseQueueBP for PromiseQueue {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn add(&mut self, promise: Promise) {
        self.queue.push(promise);
    }
}
