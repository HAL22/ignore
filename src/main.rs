use rusqlite::{Connection, Result,NO_PARAMS};
use ignore::{create_table};

fn main() -> Result<(),()>{
    
    let _databasename = "Ignore_Database";
    let _tablename = "Ignore_Table";

    let conn = Connection::open(_databasename);
    match conn{
        Ok(connection) => {
            let result = create_table(&connection, &_tablename);
            return Ok(())
        }
        Err(err) => {  
            return Err(());
        }
    }
}
