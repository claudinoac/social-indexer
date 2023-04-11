use crate::post;
use crate::reddit::RedditUser;
use serde::{Serialize, Deserialize};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::{Cursor, Write, Read};
use std::io::{Result};


#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: String,
    pub followers: i32,
    pub following: i32,
    pub description: String
} 

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            followers: 0,
            following: 0,
            created_at: "".to_string(),
            description: "".to_string(),
            username: "".to_string(),
        }
    }
}

impl User {
    fn posts(&self) -> Result<Option<post::Post>> {
        return Ok(Some(post::Post::new("template comment".to_string(), "xuxa".to_string())));
    }

    pub fn new(id: i32, username: &str, created_at: &str, followers: i32, following: i32, description: &str) -> User {
        User {
            id,
            created_at: created_at.to_string(),
            description: description.to_string(),
            username: username.to_string(),
            followers,
            following,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> (User, u64) {
        let mut cursor = Cursor::new(bytes);

        let user_id = cursor.read_u32::<LittleEndian>().unwrap() as i32;
        let followers = cursor.read_u32::<LittleEndian>().unwrap() as i32;
        let following = cursor.read_u32::<LittleEndian>().unwrap() as i32;

        let created_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut created_buffer = vec![0; created_len as usize];
        cursor.read_exact(&mut created_buffer).unwrap();
        let created_at = String::from_utf8(created_buffer).unwrap();

        let description_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut description_buffer = vec![0; description_len as usize];
        cursor.read_exact(&mut description_buffer).unwrap();
        let description = String::from_utf8(description_buffer).unwrap();

        let user_len = cursor.read_u32::<LittleEndian>().unwrap();
        let mut user_buffer = vec![0; user_len as usize];
        cursor.read_exact(&mut user_buffer).unwrap();
        let username = String::from_utf8(user_buffer).unwrap();

        return (User {
            id: user_id,
            followers,
            following,
            created_at,
            description,
            username,
        }, cursor.position());
    }

    pub fn to_bytes(&self) -> Cursor<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());

        buffer.write_u32::<LittleEndian>(self.id as u32).unwrap();
        buffer.write_u32::<LittleEndian>(self.followers as u32).unwrap();
        buffer.write_u32::<LittleEndian>(self.following as u32).unwrap();

        let created_bytes = self.created_at.as_bytes(); 
        buffer.write_u32::<LittleEndian>(created_bytes.len() as u32).unwrap();
        buffer.write_all(created_bytes).unwrap();

        let desc_bytes = self.description.as_bytes(); 
        buffer.write_u32::<LittleEndian>(desc_bytes.len() as u32).unwrap();
        buffer.write_all(desc_bytes).unwrap();

        let user_bytes = self.username.as_bytes(); 
        buffer.write_u32::<LittleEndian>(user_bytes.len() as u32).unwrap();
        buffer.write_all(user_bytes).unwrap();

        return buffer;
    }
}
