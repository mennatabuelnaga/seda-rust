use thiserror::Error;
#[derive(Error, Debug)]
pub enum NearAdapterError {
    #[error("error calling contract change method")]
    CallChangeMethod(String),
    #[error("error calling contract view method")]
    CallViewMethod(),
    #[error("time limit exceeded for the transaction to be recognized")]
    BadTransactionTimestamp(),
    #[error("could not deserialize status to string")]
    BadDeserialization(),
}
