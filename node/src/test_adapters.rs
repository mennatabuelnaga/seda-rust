// use futures::executor;
// /// TODO: Delete this and replace with actual adapters
// use rusqlite::params;
// use seda_runtime::{
//     adapters::{DatabaseAdapter, HostAdapterTypes, HttpAdapter},
//     RuntimeError,
// };
// use tokio_rusqlite::Connection;

// #[derive(Clone, Default)]
// pub struct TestAdapters;

// #[derive(Clone)]
// pub struct DatabaseTestAdapter {
//     conn: Connection,
// }
// impl Default for DatabaseTestAdapter {
//     fn default() -> Self {
//         executor::block_on(async move {
//             let conn = Connection::open("./seda_db.db3").await.expect("Couldn't open db conn");
//             conn.call(|conn| {
//                 conn.execute(
//                     "CREATE TABLE IF NOT EXISTS data (
//                                 key TEXT PRIMARY KEY,
//                                 value TEXT NOT NULL
//                             )",
//                     params![],
//                 )
//                 .expect("couldn't create db table");

//                 Ok::<_, RuntimeError>(())
//             })
//             .await
//             .expect("Couldn't execute db call");
//             DatabaseTestAdapter { conn }
//         })
//     }
// }

// #[async_trait::async_trait]
// impl DatabaseAdapter for DatabaseTestAdapter {
//     async fn get(&self, key: &str) -> Result<Option<String>, RuntimeError> {
//         let key = key.to_string();
//         let value = self
//             .conn
//             .call(move |conn| {
//                 let mut stmt = conn.prepare("SELECT value FROM data WHERE key = ?1")?;
//                 let mut retrieved: Option<String> = None;

//                 stmt.query_row([key], |row| {
//                     retrieved = row.get(0)?;
//                     Ok(())
//                 })?;
//                 Ok::<_, RuntimeError>(retrieved)
//             })
//             .await?;

//         Ok(value)
//     }

//     async fn set(&mut self, key: &str, value: &str) -> Result<(), RuntimeError> {
//         let key = key.to_string();
//         let value = value.to_string();
//         self.conn
//             .call(move |conn| {
//                 conn.execute("INSERT INTO data (key, value) VALUES (?1, ?2)", params![key, value])?;

//                 Ok::<_, RuntimeError>(())
//             })
//             .await?;

//         Ok(())
//     }
// }

// // impl Default for DatabaseTestAdapter {
// //     fn default() -> Self {
// //         // tokio::task::block_in_place(move || {
// //         //     tokio::runtime::Handle::current().block_on(async move {
// //         //         let conn =
// //         // Connection::open("./seda_db.db3").await.expect("Couldn't open db
// //         // conn");         conn.call(|conn| {
// //         //             conn.execute(
// //         //                 "CREATE TABLE IF NOT EXISTS data (
// //         //                             key TEXT PRIMARY KEY,
// //         //                             value TEXT NOT NULL
// //         //                         )",
// //         //                 params![],
// //         //             )
// //         //             .expect("couldn't create db table");

// //         //             Ok::<_, RuntimeError>(())
// //         //         })
// //         //         .await
// //         //         .expect("Couldn't execute db call");
// //         //         DatabaseTestAdapter { conn }
// //         //     })
// //         // })

// //         Self {}
// //     }
// // }

// // #[async_trait::async_trait]
// // impl DatabaseAdapter for DatabaseTestAdapter {
// //     async fn get(&self, key: &str) -> Result<Option<String>, RuntimeError> {
// //         // let key = key.to_string();
// //         // let value = self
// //         //     .conn
// //         //     .call(move |conn| {
// //         //         let mut stmt = conn.prepare("SELECT value FROM data WHERE key =
// //         // ?1")?;         let mut retrieved: Option<String> = None;

// //         //         stmt.query_row([key], |row| {
// //         //             retrieved = row.get(0)?;
// //         //             Ok(())
// //         //         })?;
// //         //         Ok::<_, RuntimeError>(retrieved)
// //         //     })
// //         //     .await?;

// //         Ok(Some("".to_string()))
// //     }

// //     async fn set(&mut self, key: &str, value: &str) -> Result<(), RuntimeError> {
// //         // let key = key.to_string();
// //         // let value = value.to_string();
// //         // self.conn
// //         //     .call(move |conn| {
// //         //         conn.execute("INSERT INTO data (key, value) VALUES (?1, ?2)",
// //         // params![key, value])?;

// //         //         Ok::<_, RuntimeError>(())
// //         //     })
// //         //     .await?;

// //         Ok(())
// //     }
// // }
// #[derive(Clone, Default)]
// pub struct HttpTestAdapter;

// #[async_trait::async_trait]
// impl HttpAdapter for HttpTestAdapter {
//     async fn fetch(&mut self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
//         reqwest::get(url).await
//     }
// }

// impl HostAdapterTypes for TestAdapters {
//     type Database = DatabaseTestAdapter;
//     type Http = HttpTestAdapter;
// }
