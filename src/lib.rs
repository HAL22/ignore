use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn create_table(connection:& Connection,tablename:&str) -> Result<()>{

    let create_table_string = format!("create table if not exists {} (
        key text primary key,
        value text not null unique
    )",tablename);

    connection.execute(
        &create_table_string,
        NO_PARAMS,
    )?;

    return Ok(())
}