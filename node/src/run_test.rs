use std::sync::Arc;

use actix::{Actor, System};
use parking_lot::RwLock;
use seda_config::{ChainConfigsInner, NodeConfigInner, P2PConfigInner};
use seda_p2p::PeerList;
use seda_runtime_sdk::{
    events::{Event, EventData},
    p2p::P2PCommand,
};
use tokio::sync::mpsc::channel;

use crate::{
    app::{App, DebugStop},
    event_queue_handler::AddEventToQueue,
    host::RuntimeAdapter,
};

#[test]
fn shared_memory_test() {
    let system = System::new();
    let p2p_config = P2PConfigInner::test_config();

    system.block_on(async {
        // let (p2p_message_sender, p2p_message_receiver) = channel::<P2PMessage>(100);
        let (p2p_command_sender, _) = channel::<P2PCommand>(100);

        let known_peers = Arc::new(RwLock::new(PeerList::from_vec(&p2p_config.p2p_known_peers)));

        let app = App::<RuntimeAdapter>::new(
            NodeConfigInner::test_config(),
            "127.0.0.1:12345",
            ChainConfigsInner::test_config(),
            p2p_command_sender,
            known_peers.clone(),
        )
        .await
        .start();

        actix::spawn(async move {
            app.send(AddEventToQueue {
                event: Event {
                    id:   "shared_memory_test".into(),
                    data: EventData::ChainTick,
                },
            })
            .await
            .unwrap();

            loop {
                app.send(DebugStop).await.unwrap();
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            }
        });
    });

    let code = system.run_with_code().unwrap();
    assert_eq!(code, 0);

    let system = System::new();
    system.block_on(async {
        // let (p2p_message_sender, p2p_message_receiver) = channel::<P2PMessage>(100);
        let (p2p_command_sender, _) = channel::<P2PCommand>(100);

        let known_peers = Arc::new(RwLock::new(PeerList::from_vec(&p2p_config.p2p_known_peers)));

        let app = App::<RuntimeAdapter>::new(
            NodeConfigInner::test_config(),
            "127.0.0.1:12345",
            ChainConfigsInner::test_config(),
            p2p_command_sender,
            known_peers.clone(),
        )
        .await
        .start();

        actix::spawn(async move {
            app.send(AddEventToQueue {
                event: Event {
                    id:   "shared_memory_test".into(),
                    data: EventData::ChainTick,
                },
            })
            .await
            .unwrap();

            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                app.send(DebugStop).await.unwrap();
            }
        });
    });

    let code = system.run_with_code().unwrap();
    assert_eq!(code, 0);
}
