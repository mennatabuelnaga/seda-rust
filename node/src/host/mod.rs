mod db_set;
mod db_get;
mod http_fetch;
pub use db_get::DatabaseGet;
pub use db_set::DatabaseSet;
pub use http_fetch::HttpFetch;


use actix::prelude::*;
use futures::executor;
use rusqlite::params;
use tokio_rusqlite::Connection;

use crate::NodeError;

// #[derive(Default)]
pub struct Host{
    db_conn: Connection
}

impl Default for Host {
    fn default() -> Self {
        executor::block_on(async move {
            let db_conn = Connection::open("./seda_db.db3").await.expect("Couldn't open db conn");        
            db_conn.call(|db_conn| {
                db_conn.execute(
                    "CREATE TABLE IF NOT EXISTS data (
                                key TEXT PRIMARY KEY,
                                value TEXT NOT NULL
                            )",
                    params![],
                )
                .expect("couldn't create db table");

                Ok::<_, NodeError>(())
            })
            .await
            .expect("Couldn't execute db call");
            Host { db_conn }
        })

    }
}




impl Actor for Host {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started host adapter");
        // connect to db stuff etc...
    }
}

impl actix::Supervised for Host {}

impl SystemService for Host {}

