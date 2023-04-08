use std::fs::{File, OpenOptions};
use std::path::{PathBuf};
use std::io::{Read, Write};
use std::io::Result;
use crate::post::{Post};
use crate::user::{User};
extern crate bytecheck;
extern crate rkyv;
use rkyv::{Archive, Deserialize, Serialize};


pub struct Table {
    name: String,
    file: File,
    file_path: String,
    item_type: String
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub enum Row {
    User(User),
    Post(Post),
}

impl Table {
    pub fn new(name: &str, path: &str, item_type: &str) -> Result<Self> {
        let file_path = PathBuf::from(path);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)?;

        Ok(Self {
            name: name.to_string(),
            file,
            file_path: path.to_string(),
            item_type: item_type.to_string(),
        })
    }

    pub fn insert<'a, 'b>(&'a mut self, row: &'b Row) -> Result<&'b Row> {
        let mut file = Table::open(&self.name, &self.file_path, &self.item_type, true); 
        let bytes = rkyv::to_bytes::<_, 256>(row).unwrap();
        let _ = file.write_all(&bytes);
        let new_row = row.clone();
        return Ok(new_row);
    }

    pub fn get<T>(&mut self, index: i32) -> Result<Row> {
        let mut file = Table::open(&self.name, &self.file_path, &self.item_type, false); 
        // let bytes =  
        // let deserialized = rkyv::from_bytes::<Row>(&bytes); TODO: load a single node from memory
        match self.item_type.as_str() {
            "user" => Ok(Row::User(User::default())),
            "post" => Ok(Row::Post(Post::default())),
            _ => panic!("Unrecognized type!"),
        }
    }

    pub fn save(&mut self, row: Row) -> Result<Row> {
        return Ok(Row::User(User::default()));
    }

    pub fn open(name: &str, path: &str, item_type: &str, append: bool) -> File {
        let file_path = PathBuf::from(path);
        let file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .append(append)
                        .open(&file_path);
        match file {
            Ok(file) => file, 
            Err(error) => panic!("Cannot load file for table {:} {:}", name, error)
        }
    }
}
