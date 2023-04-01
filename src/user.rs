use crate::tweet;
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
    fn tweets(&self) -> Result<Option<tweet::Tweet>> {
        return Ok(Some(tweet::Tweet::new(String::from("template comment"), 1)));
    }
}
