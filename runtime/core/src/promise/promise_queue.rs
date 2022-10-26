/// Acts as a wrapper around a vector of promises
/// This allows us to change to batches easily in the future
use super::Promise;

#[derive(Clone, Default)]
pub struct PromiseQueue {
    /// A list which contains batches of promises
    pub queue: Vec<Promise>,
}

impl PromiseQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn add_promise(&mut self, promise: Promise) {
        self.queue.push(promise);
    }
}
