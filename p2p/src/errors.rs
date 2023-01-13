use thiserror::Error;

#[derive(Error, Debug)]
pub enum P2PAdapterError {
    #[error("libp2p transport error: {0}")]
    Trasnport(#[from] libp2p::TransportError<std::io::Error>),
    #[error("libp2p gossip error {0}")]
    Gossip(String),
    #[error("libp2p gossip subscription error {0}")]
    GossipSubscription(#[from] libp2p::gossipsub::error::SubscriptionError),
    #[error("libp2p io error {0}")]
    Io(#[from] std::io::Error),
    #[error("libp2p multi addr error: {0}")]
    MultiAddr(#[from] libp2p::multiaddr::Error),
}

pub type Result<T, E = P2PAdapterError> = core::result::Result<T, E>;
