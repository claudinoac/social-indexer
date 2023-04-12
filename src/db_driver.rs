use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::{PathBuf, Path};
use std::io::{Cursor, Read, Write, SeekFrom, Result, Error, ErrorKind, BufReader};
use crate::user::{User};
use crate::post::{Post};
extern crate bytecheck;
extern crate rkyv;
use serde::{Deserialize, Serialize};
use byteorder::{ReadBytesExt, LittleEndian};
extern crate btree;
use btree::node_type::{KeyValuePair};
use btree::btree::{BTreeBuilder};
use btree::error::{Error as BTreeError};
use std::result::{Result as StdResult};
extern crate rsdb;



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

fn read_ne_i32(input: &[u8]) -> i32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<i32>());
    i32::from_ne_bytes(int_bytes.try_into().unwrap())
}

impl Row {
    pub fn from_bytes(bytes: &[u8], item_type: &String) -> (Self, u64) {
        return match item_type.as_str() {
            "post" => {
                let (entry, cursor) = Post::from_bytes(bytes);
                return (Row::Post(entry), cursor);
            }, 
            "user" => {
                let (entry, cursor) = User::from_bytes(bytes);
                return (Row::User(entry), cursor);
            },
            _ => panic!("Cannot decode binary data for {:}", item_type)
        };
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        return match self {
            Row::User(row) => row.to_bytes().into_inner().to_vec(),
            Row::Post(row) => row.to_bytes().into_inner().to_vec(),
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

    pub fn insert<'a, 'b>(&'a mut self, row: &'b Row) -> Result<i32> {
        let mut file = Table::open(&self.name, &self.file_path, &self.item_type, true, true); 
        let bytes = row.to_bytes();
        println!("Writing {:} bytes", bytes.len());
        file.write_all(&bytes)?;
        file.seek(SeekFrom::End(0))?;
        let position = (file.stream_position()? - bytes.len() as u64) as i32;
        match row {
            Row::User(user) => self.write_to_pk_index(&user.source_id, position),
            Row::Post(post) => self.write_to_pk_index(&post.source_id, position),
        };
        println!("Start position: {:}", position);
        return Ok(position as i32);
    }

    pub fn get(&mut self, index: i32) -> Result<Row> {
        let mut file = Table::open(&self.name, &self.file_path, &self.item_type, false, false); 
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

    pub fn get_row_by_byte_position(&mut self, file_position: i32) -> Result<Row> {
        let file = Table::open(&self.name, &self.file_path, &self.item_type, false, false); 
        let mut buffer = [0u8;  2000];
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(file_position as u64))?;
        reader.read(&mut buffer)?;
        let (entry, _) = Row::from_bytes(&buffer, &self.item_type);
        return Ok(entry)
    }

    pub fn get_row_by_source_id(&mut self, source_id: &str) -> Result<Row> {
        let tree = rsdb::Config::default()
          .path(Some(format!("./{:}_index.btree", self.item_type)))
          .tree();
        let source_bytes = source_id.clone().as_bytes();
        let byte_position = tree.get(source_bytes).unwrap();
        let mut byte_position = &byte_position[..]; 
        let row_position = read_ne_i32(byte_position);
        return self.get_row_by_byte_position(row_position);
    }

    pub fn write_to_pk_index(&mut self, source_id: &str, disk_position: i32) -> StdResult<(), BTreeError> {
        // let tree = rsdb::Config::default()
        //   .path(Some(format!("./{:}_index.btree", self.item_type)))
        //   .tree();
        // tree.set(source_id.as_bytes().to_vec(), disk_position.to_ne_bytes().to_vec());
        return Ok(());
    }

    pub fn print_pk_index_tree(&mut self) -> StdResult<(), BTreeError> {
        let tree = rsdb::Config::default()
          .path(Some(format!("./{:}_index.btree", self.item_type)))
          .tree();
        return Ok(());
    }

    pub fn all(&mut self) -> Result<Vec<Row>> {
        let mut file = Table::open(&self.name, &self.file_path, &self.item_type, false, false); 
        println!("File size: {:}", file.metadata().unwrap().len());
        let mut buffer = [0u8;  2000];
        let mut reader = BufReader::new(file);
        let mut entries: Vec<Row> = Vec::new();
        let mut buffer_offset: i64 = 0;
        reader.seek(SeekFrom::Start(0))?;
        while let Ok(bytes_read) = reader.read(&mut buffer) {
            println!("Bytes read: {:}", bytes_read);
            if bytes_read == 0 {
                break;
            }
            let (entry, cursor) = Row::from_bytes(&buffer, &self.item_type);
            entries.push(entry);
            buffer.iter_mut().for_each(|b| *b = 0);
            buffer_offset = bytes_read as i64 - cursor as i64;
            println!("buffer_offset: {:}", buffer_offset);
            if buffer_offset > 0 {
                let current_position = reader.stream_position()?;
                reader.seek(SeekFrom::Start(current_position - buffer_offset as u64))?;
            } else {
                break
            }
            println!("Cursor position: {:}", reader.stream_position()?);
        }
        println!("Loaded {:} entries", entries.len());
        return Ok(entries); 
    }

    pub fn open(name: &str, path: &str, item_type: &str, append: bool, write: bool) -> File {
        let file_path = PathBuf::from(path);
        let file = OpenOptions::new()
                        .read(true)
                        .write(write)
                        .append(append)
                        .open(&file_path);
        match file {
            Ok(file) => file, 
            Err(error) => panic!("Cannot load file for table {:} {:}", name, error)
        }
    }
}
