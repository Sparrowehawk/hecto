use std::{io::Error, fs::read_to_string};

#[derive(Default)]
pub struct Buffer{
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(file_name: &str) -> Result<Self, Error>{
        let path = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in path.lines(){
            lines.push(String::from(value));
        }    

        Ok(Self{lines})
    }
}