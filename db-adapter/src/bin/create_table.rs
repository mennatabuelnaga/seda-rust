
use rusqlite::{Connection, Result, params};


fn main() -> Result<()> {
    let conn = Connection::open("./seda_db.db3")?;

    conn.execute(
        "CREATE TABLE node (
            id INTEGER PRIMARY KEY,
            socket_address TEXT NOT NULL,
            owner TEXT NOT NULL
        )",
        (),
    )?;
    
   
    Ok(())
}