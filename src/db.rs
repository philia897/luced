use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub fn establish_connection(path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(path)?;
    Ok(conn)
}
