use rusqlite::{Connection, Result,NO_PARAMS};
use std::fs::File;
use std::env;
use std::fs::OpenOptions;

use lib::event_driver;

fn main() -> Result<()>{

    let args: Vec<String> = env::args().collect();

    if args.len()>=0{
        let _databasename = "ignore.db";
        let _tablename = "test";
        let path = "/Users/thethelafaltein/Desktop/Projects/rustlangproj/ignore/lable";
    
        let File = match OpenOptions::new().read(true).write(true).create(true).append(true).open(path){
                    Result::Ok(file) => {file},
                    Result::Err(e) => {panic!()}
        };
        let conn = Connection::open(&_databasename)?;
    
        let mut event = lib::event_driver::MyEventDriver::new(&conn,_tablename.to_string(),File);

        event.event_handler(env::args(), 3);
    }
    


    

    

    







    Ok(())
}
