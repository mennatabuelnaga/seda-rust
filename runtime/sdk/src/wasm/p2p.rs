use super::Promise;
use crate::{P2PBroadcastAction, PromiseAction};

pub fn p2p_broadcast_message(data: Vec<u8>) -> Promise {
    Promise::new(PromiseAction::P2PBroadcast(P2PBroadcastAction { data }))
}
