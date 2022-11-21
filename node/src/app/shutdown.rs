use actix::{Context, Handler, Message, System};

use super::App;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Shutdown;

impl Handler<Shutdown> for App {
    type Result = ();

    fn handle(&mut self, _msg: Shutdown, _ctx: &mut Context<Self>) {
        // Close RPC server
        if let Err(error) = self.rpc_server.stop() {
            println!("Some error happened while closing RPC: {}", error);
        }

        // Close actix system
        System::current().stop();
    }
}
