use rusqlite::{Connection, Result,Statement};
use std::fs::File;
use std::env;

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

    pub fn generate_gitignore_db(& mut self,keys:&Vec<String>) -> Result<()>{
        let mut files_holder :Vec<String> = Vec::new();
        self.dbcontext.connection.execute_batch("BEGIN TRANSACTION;")?;
        for key in keys{
          //  println!("{:?}",key);
           let mut vec :Vec<String> =  self.dbcontext.read_gitignorefile(&key)?;
           files_holder.push(vec.remove(0));
        }
        self.dbcontext.connection.execute_batch("COMMIT TRANSACTION;")?;
        let result = self.filecontext.make_or_amend_gitignore_using_files(&files_holder);
        return Ok(());
    }

    pub fn generate_gitignore_userinput(& mut self,user_input:&String){
        let result = self.filecontext.make_or_amend_gitignore_using_userinput(user_input);
    }

    pub fn event_handler(& mut self,mut args: env::Args,size:i32) -> Result<(),String>{

        if size<2{
            return Err(String::from("Not enough inputs"));
        }else{

            args.next();
        }

        let mut keys:Vec<String> = Vec::new();

        for arg in args{

            keys.push(arg);

        }

        //println!("{:?}",keys);

        self.generate_gitignore_db(&keys);

        
        return Ok(());

    }

    




}