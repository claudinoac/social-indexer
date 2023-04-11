extern crate bytecheck;
extern crate byteorder;
use serde::{Deserialize, Serialize};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::{Cursor, Write, Read};

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub date: String,
    pub content: String,
    pub source_id: String,
    pub reactions: i32,
    pub shares: i32,
    pub comments: i32,
    pub username: String,
    pub url: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            date: String::from("today"),
            content: String::from(""),
            source_id: String::from("abcd"),
            url: String::from(""),
            username: String::from(""),
            reactions: 0,
            shares: 0,
            comments: 0,
        }
    }
}

impl Post {
    pub fn new(content: String, username: String) -> Post {
        Post {
            content,
            username,
            ..Post::default()
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> (Post, u64) {
        let mut cursor = Cursor::new(bytes);

        let date_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut date_buffer = vec![0; date_len as usize];
        cursor.read_exact(&mut date_buffer).unwrap();
        let date = String::from_utf8(date_buffer).unwrap();

        let source_id_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut source_id_buffer = vec![0; source_id_len as usize];
        cursor.read_exact(&mut source_id_buffer).unwrap();
        let source_id = String::from_utf8(source_id_buffer).unwrap();

        let content_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut content_buffer = vec![0; content_len as usize];
        cursor.read_exact(&mut content_buffer).unwrap();
        let content = String::from_utf8(content_buffer).unwrap();

        let url_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut url_buffer = vec![0; url_len as usize];
        cursor.read_exact(&mut url_buffer).unwrap();
        let url = String::from_utf8(url_buffer).unwrap();

        let user_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut user_buffer = vec![0; user_len as usize];
        cursor.read_exact(&mut user_buffer).unwrap();
        let username = String::from_utf8(user_buffer).unwrap();

        let reactions = cursor.read_u32::<LittleEndian>().unwrap();
        let shares = cursor.read_u32::<LittleEndian>().unwrap();
        let comments = cursor.read_u32::<LittleEndian>().unwrap();

        return (Post {
            reactions: reactions as i32,
            shares: shares as i32,
            comments: comments as i32,
            date,
            source_id,
            content,
            url,
            username,
        }, cursor.position());
    }

    pub fn to_bytes(&self) -> Cursor<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());
        let date_bytes = self.date.as_bytes(); 
        buffer.write_u32::<LittleEndian>(date_bytes.len() as u32).unwrap();
        buffer.write_all(date_bytes).unwrap();

        let source_id_bytes = self.source_id.as_bytes();
        buffer.write_u32::<LittleEndian>(source_id_bytes.len() as u32).unwrap();
        buffer.write_all(source_id_bytes).unwrap();

        let content_bytes = self.content.as_bytes();
        buffer.write_u32::<LittleEndian>(content_bytes.len() as u32).unwrap();
        buffer.write_all(content_bytes).unwrap();

        let url_bytes = self.url.as_bytes();
        buffer.write_u32::<LittleEndian>(url_bytes.len() as u32).unwrap();
        buffer.write_all(url_bytes).unwrap();

        let username_bytes = self.username.as_bytes();
        buffer.write_u32::<LittleEndian>(username_bytes.len() as u32).unwrap();
        buffer.write_all(username_bytes).unwrap();

        buffer.write_u32::<LittleEndian>(self.reactions as u32).unwrap();
        buffer.write_u32::<LittleEndian>(self.shares as u32).unwrap();
        buffer.write_u32::<LittleEndian>(self.comments as u32).unwrap();
        buffer
    }
}
