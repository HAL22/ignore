use rusqlite::{Connection, Result,Statement};

pub struct MyDbContext<'a>{
    pub connection: &'a Connection,
    pub tablename: String,
    pub make_table_statement: Option<Statement<'a>>,
    pub create_gitignorefile_statement: Option<Statement<'a>>,
    pub read_gitignorefile_statement: Option<Statement<'a>>,
    pub update_gitignorefile_statement: Option<Statement<'a>>,
    pub delete_gitignorefile_statement: Option<Statement<'a>>,
}

impl<'a> MyDbContext<'a>{
    pub fn new(connection: &'a Connection,tablename:String) -> Self
    {
        return MyDbContext{
            connection,
            tablename,
            make_table_statement: None,
            create_gitignorefile_statement: None,
            read_gitignorefile_statement: None,
            update_gitignorefile_statement: None,
            delete_gitignorefile_statement: None,
        }
    }

    pub fn make_table(&mut self) -> Result<()>{
        if let None = &self.make_table_statement {
            let stmt = self.connection.prepare("create table if not exists :table (
                key text primary key,
                value text not null unique
            )")?;
            self.make_table_statement = Some(stmt);
        };
        self.make_table_statement.as_mut().unwrap().execute_named(&[(":table",&self.tablename)])?;
        return Ok(());
    }

    pub fn create_gitignorefile(&mut self,key: &String,value: &String) -> Result<i64>{
        if let None = &self.create_gitignorefile_statement{
            let stmt = self.connection.prepare("insert into :table (key, value) values (:key, :value)")?;
            self.create_gitignorefile_statement = Some(stmt);
        };
        self.create_gitignorefile_statement.as_mut().unwrap().execute_named(
            &[(":table",&self.tablename),(":key",key),(":value",value)]
        )?;
        return Ok(self.connection.last_insert_rowid());
    }

    pub fn read_gitignorefile(&mut self,key: &String) -> Result<Vec<String>>{
   
        if let None = &self.read_gitignorefile_statement{  
            let mut stmt = self.connection.prepare(&format!("SELECT value FROM {} WHERE key = :key",&self.tablename)[..])?;
            self.read_gitignorefile_statement = Some(stmt);
        }
             
        let mut values = Vec::new();

        let rows = self.read_gitignorefile_statement.as_mut().unwrap().query_map_named(&[(":key",&String::from(key))],|row|{row.get(0)})?;
        
        for row in rows{
            values.push(row?);
        }        
        return Ok(values);
    }

    pub fn update_gitignorefile(& mut self,key: &String,new_value: &String) -> Result<()>{
        if let None = &self.update_gitignorefile_statement{
            let stmt = self.connection.prepare("update :table set value = :value where key = :key")?;
            self.update_gitignorefile_statement = Some(stmt);
        }
        self.update_gitignorefile_statement.as_mut().unwrap().execute_named(
            &[(":table",&self.tablename),(":key",key),(":value",new_value)]
        )?;
        return Ok(());
    }

    pub fn delete_gitignorefile(& mut self,key: &String) -> Result<()>{
        if let None = &self.delete_gitignorefile_statement{
            let stmt = self.connection.prepare("delete from :table where key = :key")?;
            self.delete_gitignorefile_statement = Some(stmt);
        }
        self.delete_gitignorefile_statement.as_mut().unwrap().execute_named(
            &[(":table",&self.tablename),(":key",key)]
        )?;
        return Ok(());
    }

}