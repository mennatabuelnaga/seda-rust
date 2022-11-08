
use rusqlite::{Connection, Result, params};


#[derive(Debug)]
struct Node {
    id: u64,
    socket_address: String,
    owner: String
}

fn main() -> Result<()> {
    let conn = Connection::open("./seda_db.db3")?;

    let node_id = 3;

    conn.execute(
        "DELETE FROM node WHERE id = (?1)",
        params![node_id],
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