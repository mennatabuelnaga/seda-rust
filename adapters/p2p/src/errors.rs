use thiserror::Error;

#[derive(Error, Debug)]
pub enum P2PAdapterError {
    #[error("libp2p transport error")]
    P2PTrasnport(#[from] libp2p::TransportError<std::io::Error>),
    #[error("libp2p multi addr error")]
    P2PMultiAddr(#[from] libp2p::multiaddr::Error),
}

pub type Result<T, E = P2PAdapterError> = core::result::Result<T, E>;
