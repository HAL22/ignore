use rusqlite::{Connection, Result,Statement};

pub struct InputChecker<'a>{
    pub connection: &'a Connection,
    pub tablename: String,
    pub input_checker_create: Option<Statement<'a>>,
    pub input_checker_read: Option<Statement<'a>>,
    pub input_checker_update: Option<Statement<'a>>,
    pub input_checker_delete: Option<Statement<'a>>,
}

impl<'a> InputChecker<'a>{
    pub fn new(connection: &'a Connection,tablename:String) -> Self{
        return InputChecker{
            connection,
            tablename,
            input_checker_create: None,
            input_checker_read: None,
            input_checker_update: None,
            input_checker_delete: None,
        }
    }

}