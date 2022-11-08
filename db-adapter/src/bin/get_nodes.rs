
use rusqlite::{Connection, Result, params};


#[derive(Debug)]
struct Node {
    id: u64,
    socket_address: String,
    owner: String
}

fn main() -> Result<()> {
    let conn = Connection::open("./seda_db.db3")?;

  
    let offset = 0;
    let limit = 100;
    let mut stmt = conn.prepare("SELECT MAX(id) FROM node")?;
    let mut last_node_id: u64 = 100;
    stmt.query_row([], |row| {
        last_node_id = row.get(0)?;
        Ok(())
    })?;

    let mut stmt = conn.prepare("SELECT id, socket_address, owner FROM node WHERE id <= ?1 LIMIT ?2")?;
    let node_iter = stmt.query_map([last_node_id - offset, limit], |row| {
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