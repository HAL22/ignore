use rusqlite::{Connection, Result,Statement,NO_PARAMS};


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

        let create_table_string = format!("create table if not exists {} (
            key text primary key,
            value text not null unique
        )",&self.tablename);

        if let None = &self.make_table_statement {

            let stmt = self.connection.prepare(&create_table_string)?;

            self.make_table_statement = Some(stmt);
        };

        self.make_table_statement.as_mut().unwrap().execute(NO_PARAMS)?;

        return Ok(());

    }

    pub fn create_gitignorefile(&mut self,key: &String,value: &String) -> Result<i64>{

        let create_gitignorefile_string = format!("insert into {} (key, value) values (:key, :value)",&self.tablename);

        if let None = &self.create_gitignorefile_statement{

            let stmt = self.connection.prepare(&create_gitignorefile_string)?;

            self.create_gitignorefile_statement = Some(stmt);
        };

        self.create_gitignorefile_statement.as_mut().unwrap().execute_named(
            &[(":key",key),(":value",value)]
        )?;

        return Ok(self.connection.last_insert_rowid());

    }

    pub fn read_gitignorefile(&mut self,key: &String) -> Result<Vec<String>>{

        let read_gitignorefile_string = format!("select value from {} where key = :key",&self.tablename);

        if let None = &self.read_gitignorefile_statement{

            let stmt = self.connection.prepare(&read_gitignorefile_string)?;

            self.read_gitignorefile_statement = Some(stmt);
        }

        let mut values = Vec::new();

        let rows = self.read_gitignorefile_statement.as_mut().unwrap().query_map_named(&[(":key",key)],|row|{row.get(0)})?;

        for row in rows{
            values.push(row?);
        }

        return Ok(values);

    }

    pub fn update_gitignorefile(& mut self,key: &String,new_value: &String) -> Result<()>{

        let update_gitignorefile_string = format!("update {} set value = :value where key = :key",&self.tablename);

        if let None = &self.update_gitignorefile_statement{

            let stmt = self.connection.prepare(&update_gitignorefile_string)?;

            self.update_gitignorefile_statement = Some(stmt);
        }

        self.update_gitignorefile_statement.as_mut().unwrap().execute_named(
            &[(":key",key),(":value",new_value)]
        )?;

        return Ok(());

    }

    pub fn delete_gitignorefile(& mut self,key: &String) -> Result<()>{

        let delete_gitignorefile_string = format!("delete from {} where key = :key",&self.tablename);

        if let None = &self.delete_gitignorefile_statement{

            let stmt = self.connection.prepare(& delete_gitignorefile_string)?;

            self.delete_gitignorefile_statement = Some(stmt);

        }

        self.delete_gitignorefile_statement.as_mut().unwrap().execute_named(
            &[(":key",key)]
        )?;

        return Ok(());

    }


}
