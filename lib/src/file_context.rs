use std::fs::File;
use std::io::{BufWriter, Write,BufRead, BufReader,Error};

pub struct MyFileContext<'a>{
    pub gitignore_file: & 'a File,
}

impl<'a> MyFileContext<'a>{
    pub fn new(file: &'a File) -> Self {
       return  MyFileContext{
            gitignore_file:file
        }
    }

    pub fn make_or_amend_gitignore_using_files(& mut self,ignores: &Vec<String>) ->  Result<(), Error>{ 
        let mut string_result: String = String::from("");
        let ignores_iter = ignores.iter();
        for ignore_file in ignores_iter{
            let file = File::open(ignore_file)?;
            let buf_reader = BufReader::new(file);
            for line in buf_reader.lines() {
                string_result = format!("{}\n{}",string_result,line?);
            }
        }
        let mut gitignore_buff_writer = BufWriter::new(self.gitignore_file);
        gitignore_buff_writer.write_all(string_result.as_bytes());
        return Ok(());
    }

    pub fn make_or_amend_gitignore_using_userinput(& mut self,input: &String) ->  Result<(), Error>{
        let mut gitignore_buff_writer = BufWriter::new(self.gitignore_file);
        gitignore_buff_writer.write_all(input.as_bytes());
        return Ok(())
    }
    
}