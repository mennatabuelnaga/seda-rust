use rusqlite::params;
use serde::{Deserialize, Serialize};

use actix::prelude::*;

use crate::NodeError;

use super::Host;




#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), NodeError>")]
pub struct DatabaseSet {
    pub key: String,
    pub value: String,
}


impl Handler<DatabaseSet> for Host {
    type Result = ResponseActFuture<Self, Result<(), NodeError>>;

    fn handle(&mut self, msg: DatabaseSet, ctx: &mut Self::Context) -> Self::Result {
        let key = msg.key.to_string();
        let value = msg.value.to_string();
        let db_conn = self.db_conn.clone();
       
        let fut = async move {
            
        db_conn
            .call(move |conn| {
                conn.execute("INSERT INTO data (key, value) VALUES (?1, ?2)", params![key, value])?;

                Ok::<_, NodeError>(())
            })
            .await?;

        Ok(())
        };

        Box::pin(fut.into_actor(self))
    }
}
