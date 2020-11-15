use rusqlite::{Connection, Result,Statement};
use std::fs::File;

pub struct MyEventDriver<'a>{
    pub dbcontext: crate::db_context::MyDbContext<'a>,
    pub filecontext: crate::file_context::MyFileContext<'a>,
}

impl<'a> MyEventDriver <'a>{

    pub fn new(connection: &'a Connection,tablename:String,file: &'a File) -> Self{
        return MyEventDriver{
            dbcontext: crate::db_context::MyDbContext::new(connection, tablename),
            filecontext: crate::file_context::MyFileContext::new(file)
        }
    }

    


}