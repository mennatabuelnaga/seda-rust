use rusqlite::{params, Connection};
use seda_runtime_macros::Adapter;

use super::RuntimeError;
use crate::{DatabaseAdapter, HostAdapterTypes, HttpAdapter};

#[derive(Clone, Default, Adapter)]
#[adapter(
	database = DatabaseTestAdapter,
	http = HttpTestAdapter,
)]
pub struct TestAdapters;

#[derive(Clone, Default)]
pub struct DatabaseTestAdapter {}

impl DatabaseAdapter for DatabaseTestAdapter {
    fn get(&self, conn: &Connection, key: &str) -> Result<Option<String>, RuntimeError> {
        let key = key.to_string();
        let mut stmt = conn.prepare("SELECT value FROM data WHERE key = ?1")?;
        let mut value: Option<String> = None;
        stmt.query_row([key], |row| {
            value = row.get(0)?;
            Ok(())
        })?;
        Ok(value)
    }

    fn set(&mut self, conn: &Connection, key: &str, value: &str) -> Result<(), RuntimeError> {
        let key = key.to_string();
        let value = value.to_string();
        conn.execute("INSERT INTO data (key, value) VALUES (?1, ?2)", params![key, value])?;

        Ok(())
    }

    fn connect(&mut self) -> Result<Connection, RuntimeError> {
        let conn = Connection::open("./seda_db.db3")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            (),
        )?;

        Ok(conn)
    }

    // fn get_all(&self) -> HashMap<String, String> {
    //     self.data.clone()
    // }
}

#[derive(Clone, Default)]
pub struct HttpTestAdapter;

#[async_trait::async_trait]
impl HttpAdapter for HttpTestAdapter {
    async fn fetch(&mut self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        reqwest::get(url).await
    }
}
