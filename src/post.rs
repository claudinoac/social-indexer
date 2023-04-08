extern crate bytecheck;
extern crate rkyv;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as JSONDeserialize, Serialize as JSONSerialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, JSONSerialize, JSONDeserialize)]
pub struct Post {
    pub date: String,
    pub content: String,
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
            content: content,
            ..Post::default()
        }
    }
}
