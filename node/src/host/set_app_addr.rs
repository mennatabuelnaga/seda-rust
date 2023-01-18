use actix::{Addr, Handler, Message};
use seda_runtime::HostAdapter;

use super::Host;
use crate::app::App;

/// We need to set the app address in order to access the event queue
/// The VM has the ability to add events to this queue (resolve dr, resolve
/// block, etc)
#[derive(Message)]
#[rtype(result = "()")]
pub struct SetAppAddress<HA: HostAdapter> {
    pub address: Addr<App<HA>>,
}

impl<HA: HostAdapter> Handler<SetAppAddress<HA>> for Host<HA> {
    type Result = ();

    fn handle(&mut self, msg: SetAppAddress<HA>, _ctx: &mut Self::Context) -> Self::Result {
        self.app_actor_addr = Some(msg.address);
    }
}
