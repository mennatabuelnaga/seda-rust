use serde::{Deserialize, Serialize};

use actix::prelude::*;

use super::Host;



#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "String")]
pub struct HttpFetch {
    pub url: String,
}

impl Handler<HttpFetch> for Host {
    type Result = ResponseActFuture<Self, String>;

    fn handle(&mut self, msg: HttpFetch, ctx: &mut Self::Context) -> Self::Result {
        println!("Heyhhehhehehehe");

        let fut = async {
            let x = reqwest::get(msg.url)
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            return x;
        };

       
        Box::pin(fut.into_actor(self))
    }
}
