
use rusqlite::{Connection, Result, params};


#[derive(Debug)]
struct Node {
    id: u64,
    socket_address: String,
    owner: String
}

fn main() -> Result<()> {
    let conn = Connection::open("./seda_db.db3")?;

  
    
    let socket_address = "127.0.0.1:8000".to_string();
    let owner = "mennat.testnet".to_string();

    conn.execute(
        "INSERT INTO node (socket_address, owner) VALUES (?1, ?2)",
        params![socket_address, owner],
    )?;

    let mut stmt = conn.prepare("SELECT id, socket_address, owner FROM node")?;
    let node_iter = stmt.query_map([], |row| {
        Ok(Node {
            id: row.get(0)?,
            socket_address: row.get(1)?,
            owner: row.get(2)?,

        })
    })?;

    for node in node_iter {
        println!("Found node {:?}", node.unwrap());
    }
    Ok(())
}