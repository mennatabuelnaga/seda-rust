use std::time::Duration;

use libp2p::{
    core::{muxing::StreamMuxerBox, transport, transport::upgrade::Version},
    identity,
    noise,
    tcp::{GenTcpConfig, TcpTransport},
    yamux::YamuxConfig,
    PeerId,
    Transport,
};

/// Builds the transport that serves as a common ground for all connections.
pub fn build_tcp_transport(key_pair: identity::Keypair) -> transport::Boxed<(PeerId, StreamMuxerBox)> {
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&key_pair)
        .unwrap();
    let noise_config = noise::NoiseConfig::xx(noise_keys).into_authenticated();
    let yamux_config = YamuxConfig::default();

    TcpTransport::new(GenTcpConfig::default().nodelay(true))
        .upgrade(Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux_config)
        .timeout(Duration::from_secs(20))
        .boxed()
}
