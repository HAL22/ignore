use std::fs::File;

pub struct MyFileContext<'a>{
    pub gitignore_file: & 'a File,
}

impl<'a> MyFileContext<'a>{
    pub fn new(file: &'a File) -> Self {
       return  MyFileContext{
            gitignore_file:file
        }
    }

    pub fn make_or_amend_gitignore(ignores: &Vec<String>){

        let ignores_iter = ignores.iter();

        for f in ignores_iter{

            

        }



    }


}