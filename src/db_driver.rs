use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::{PathBuf};
use std::io::{Cursor, Read, Write, SeekFrom, Result, Error, ErrorKind, BufReader};
use crate::user::{User};
use crate::post::{Post};
extern crate bytecheck;
extern crate rkyv;
use serde::{Deserialize, Serialize};


pub struct Table {
    name: String,
    file: File,
    file_path: String,
    item_type: String
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Row {
    User(User),
    Post(Post),
}

impl Row {
    pub fn from_bytes(bytes: &[u8], item_type: &String) -> (Self, u64) {
        return match item_type.as_str() {
            "post" => {
                let (entry, cursor) = Post::from_bytes(bytes);
                return (Row::Post(entry), cursor);
            },
            _ => panic!("Cannot decode binary data for {:}", item_type)
        };
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        return match self {
            Row::Post(row) => row.to_bytes().into_inner(),
            _ => panic!("well")
        };
    }
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
        let bytes = row.to_bytes();
        println!("Writing {:} bytes", bytes.len());
        let _ = file.write_all(&bytes);
        let new_row = row.clone();
        return Ok(new_row);
    }

    pub fn get(&mut self, index: i32) -> Result<Row> {
        let mut file = Table::open(&self.name, &self.file_path, &self.item_type, false); 
        let mut buffer = [0u8;  2000];
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(0))?;
        let mut idx = 1;
        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            let (entry, cursor) = Row::from_bytes(&buffer, &self.item_type);
            if idx == index {
                return Ok(entry);
            }
            idx += 1;
            buffer.iter_mut().for_each(|b| *b = 0);
            let buffer_offset = cursor as i64 - bytes_read as i64;
            reader.seek_relative(buffer_offset)?;
        }
        Err(Error::new(ErrorKind::Other, "not found."))
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
