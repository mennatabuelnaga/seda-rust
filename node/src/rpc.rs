use std::{str::FromStr, sync::Arc};

use actix::prelude::*;
use jsonrpsee::{
    core::{async_trait, Error},
    proc_macros::rpc,
    server::{ServerBuilder, ServerHandle},
};
use parking_lot::RwLock;
use seda_p2p::{
    libp2p::{Multiaddr, PeerId},
    PeerList,
};
use seda_runtime::HostAdapter;
use seda_runtime_sdk::{
    events::{Event, EventData},
    p2p::{AddPeerCommand, P2PCommand, RemovePeerCommand},
};
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tracing::debug;

use crate::runtime_job::{RuntimeJob, RuntimeWorker};

#[rpc(server)]
pub trait Rpc {
    #[method(name = "cli")]
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error>;

    #[method(name = "add_peer")]
    async fn add_peer(&self, multi_addr: String) -> Result<(), Error>;

    #[method(name = "list_peers")]
    async fn list_peers(&self) -> Result<Value, Error>;

    #[method(name = "remove_peer")]
    async fn remove_peer(&self, peer_id: String) -> Result<(), Error>;

    #[method(name = "discover_peers")]
    async fn discover_peers(&self) -> Result<(), Error>;
}

pub struct CliServer<HA: HostAdapter> {
    runtime_worker:             Addr<RuntimeWorker<HA>>,
    p2p_command_sender_channel: Sender<P2PCommand>,
    known_peers:                Arc<RwLock<PeerList>>,
}

#[async_trait]
impl<HA: HostAdapter> RpcServer for CliServer<HA> {
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error> {
        debug!("{:?}", &args);

        let result = self
            .runtime_worker
            .send(RuntimeJob {
                event: Event {
                    id:   "test".to_string(),
                    data: EventData::CliCall(args),
                },
            })
            .await
            .map_err(|err| Error::Custom(err.to_string()))?;

        Ok(result.map_err(|err| Error::Custom(err.to_string()))?.vm_result.stderr)
    }

    async fn add_peer(&self, multi_addr: String) -> Result<(), Error> {
        // To check before hand if the input is valid
        if let Err(err) = Multiaddr::from_str(&multi_addr) {
            return Err(Error::Custom(err.to_string()));
        }

        self.p2p_command_sender_channel
            .send(P2PCommand::AddPeer(AddPeerCommand { multi_addr }))
            .await
            .map_err(|err| Error::Custom(err.to_string()))?;

        Ok(())
    }

    async fn list_peers(&self) -> Result<Value, Error> {
        let peer_list = self.known_peers.read();
        let result = peer_list.get_json();

        Ok(result)
    }

    async fn remove_peer(&self, peer_id: String) -> Result<(), Error> {
        if let Err(err) = PeerId::from_str(&peer_id) {
            return Err(Error::Custom(err.to_string()));
        }

        self.p2p_command_sender_channel
            .send(P2PCommand::RemovePeer(RemovePeerCommand { peer_id }))
            .await
            .map_err(|err| Error::Custom(err.to_string()))?;

        Ok(())
    }

    async fn discover_peers(&self) -> Result<(), Error> {
        self.p2p_command_sender_channel
            .send(P2PCommand::DiscoverPeers)
            .await
            .map_err(|err| Error::Custom(err.to_string()))?;

        Ok(())
    }
}
pub struct JsonRpcServer {
    handle: ServerHandle,
}

impl JsonRpcServer {
    pub async fn start<HA: HostAdapter>(
        runtime_worker: Addr<RuntimeWorker<HA>>,
        addrs: &str,
        p2p_command_sender_channel: Sender<P2PCommand>,
        known_peers: Arc<RwLock<PeerList>>,
    ) -> Result<Self, Error> {
        let server = ServerBuilder::default().build(addrs).await?;
        let rpc = CliServer {
            runtime_worker,
            p2p_command_sender_channel,
            known_peers,
        };
        let handle = server.start(rpc.into_rpc())?;

        Ok(Self { handle })
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.handle.clone().stop()?;

        Ok(())
    }
}
