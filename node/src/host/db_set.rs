use actix::prelude::*;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use super::Host;
use crate::NodeError;

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), NodeError>")]
pub struct DatabaseSet {
    pub key:   String,
    pub value: String,
}

impl Handler<DatabaseSet> for Host {
    type Result = ResponseActFuture<Self, Result<(), NodeError>>;

    fn handle(&mut self, msg: DatabaseSet, _ctx: &mut Self::Context) -> Self::Result {
        let db_conn = self.db_conn.clone();

        let fut = async move {
            db_conn
                .call(move |conn| {
                    conn.execute(
                        "INSERT INTO data (key, value) VALUES (?1, ?2)",
                        params![msg.key, msg.value],
                    )?;

                    Ok::<_, NodeError>(())
                })
                .await?;

            Ok(())
        };

        Box::pin(fut.into_actor(self))
    }
}
