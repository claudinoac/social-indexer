use crate::post;
use serde::{Serialize, Deserialize};
use std::io::{Result};


#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    id: i32,
    username: String,
    created_at: String,
    followers: i32,
    following: i32,
    description: String
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
}
