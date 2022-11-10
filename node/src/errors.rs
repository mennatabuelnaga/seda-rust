use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("libp2p transport error")]
    P2PTrasnport(#[from] libp2p::TransportError<std::io::Error>),
    #[error("libp2p multi addr error")]
    P2PMultiAddr(#[from] libp2p::multiaddr::Error),
    // libp2p::multiaddr::Error
    #[error("io error")]
    Io(#[from] std::io::Error),
}

pub type Result<T, E = NodeError> = core::result::Result<T, E>;
