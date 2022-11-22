use rusqlite::params;
use tokio_rusqlite::Connection;

use crate::{DatabaseAdapter, Result, StorageAdapterError};

#[derive(Clone)]
pub struct DatabaseTestAdapter {
    conn: Connection,
}

impl Default for DatabaseTestAdapter {
    fn default() -> Self {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                let conn = Connection::open("./seda_db.db3").await.expect("Couldn't open db conn");
                conn.call(|conn| {
                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS data (
                                    key TEXT PRIMARY KEY,
                                    value TEXT NOT NULL
                                )",
                        params![],
                    )
                    .expect("couldn't create db table");

                    Ok::<_, StorageAdapterError>(())
                })
                .await
                .expect("Couldn't execute db call");
                DatabaseTestAdapter { conn }
            })
        })
    }
}

#[async_trait::async_trait]
impl DatabaseAdapter for DatabaseTestAdapter {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        let key = key.to_string();
        let value = self
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare("SELECT value FROM data WHERE key = ?1")?;
                let mut retrieved: Option<String> = None;

                stmt.query_row([key], |row| {
                    retrieved = row.get(0)?;
                    Ok(())
                })?;
                Ok::<_, StorageAdapterError>(retrieved)
            })
            .await?;

        Ok(value)
    }

    async fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let key = key.to_string();
        let value = value.to_string();
        self.conn
            .call(move |conn| {
                conn.execute("INSERT INTO data (key, value) VALUES (?1, ?2)", params![key, value])?;

                Ok::<_, StorageAdapterError>(())
            })
            .await?;

        Ok(())
    }
}
