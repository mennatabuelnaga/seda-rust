use actix::prelude::*;
use seda_runtime::HostAdapter;
use serde::{Deserialize, Serialize};

use super::Host;

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "String")]
pub struct HttpFetch {
    pub url: String,
}

impl<HA: HostAdapter> Handler<HttpFetch> for Host<HA> {
    type Result = ResponseActFuture<Self, String>;

    fn handle(&mut self, msg: HttpFetch, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async { reqwest::get(msg.url).await.unwrap().text().await.unwrap() };

        Box::pin(fut.into_actor(self))
    }
}
