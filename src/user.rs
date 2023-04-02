use crate::post;
use std::io::{Result};

pub struct User {
    id: i32,
    username: String,
    created_at: String,
    followers: i32,
    following: i32,
    description: String
}

impl User {
    fn posts(&self) -> Result<Option<post::Post>> {
        return Ok(Some(post::Post::new(String::from("template comment"), 1)));
    }
}

