use actix::{Context, Handler, Message, System};
use tracing::error;
use seda_chain_adapters::MainChainAdapterTrait;

use super::App;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Shutdown;

impl<MC> Handler<Shutdown> for App<MC>
where MC: MainChainAdapterTrait {
    type Result = ();

    fn handle(&mut self, _msg: Shutdown, _ctx: &mut Self::Context) {
        // Close RPC server
        if let Err(error) = self.rpc_server.stop() {
            error!("Some error happened while closing RPC: {}", error);
        }


        // Close actix system
        System::current().stop();
    }
}
