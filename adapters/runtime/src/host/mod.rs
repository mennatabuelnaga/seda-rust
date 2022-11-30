mod db_get;
pub use db_get::*;

mod db_set;
pub use db_set::*;

mod http_fetch;
use actix::prelude::*;
use futures::executor;
pub use http_fetch::*;
use rusqlite::params;
use tokio_rusqlite::Connection;

use crate::RuntimeAdapterError;

mod chain_change;
pub use chain_change::ChainChange;
mod chain_view;
pub use chain_view::ChainView;

pub struct Host {
    db_conn: Connection,
}

impl Default for Host {
    fn default() -> Self {
        executor::block_on(async move {
            let db_conn = Connection::open("./seda_db.db3").await.expect("Couldn't open db conn");

            db_conn
                .call(|db_conn| {
                    db_conn
                        .execute(
                            "CREATE TABLE IF NOT EXISTS data (
                                key TEXT,
                                value TEXT NOT NULL
                            )",
                            params![],
                        )
                        .expect("couldn't create db table");

                    Ok::<_, RuntimeAdapterError>(())
                })
                .await
                .expect("Couldn't execute db call");

            Host { db_conn}
        })
    }
}

impl Actor for Host {
    type Context = Context<Self>;
}

impl actix::Supervised for Host {}

impl SystemService for Host {}
