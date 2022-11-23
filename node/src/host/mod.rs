use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Host;

impl Actor for Host {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started host adapter");
        // connect to db stuff etc...
    }
}

impl actix::Supervised for Host {}

impl SystemService for Host {}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "String")]
pub struct DatabaseGet {
    pub key: String,
}

impl Handler<DatabaseGet> for Host {
    type Result = ResponseActFuture<Self, String>;

    fn handle(&mut self, msg: DatabaseGet, ctx: &mut Self::Context) -> Self::Result {
        println!("Heyhhehhehehehe");

        let fut = async {
            let x = reqwest::get("https://swapi.dev/api/people/2/")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            return x;
        };

        // let result = futures::executor::block_on(fut);
        // let x = ctx.wait(fut.into_actor(self));

        // println!("{:?}", result);
        // "Hey".to_string()

        Box::pin(fut.into_actor(self))
    }
}
