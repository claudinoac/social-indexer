use roux::{Reddit};
use reqwest::blocking::{Client as RequestClient};
use serde::{Deserialize, Serialize};
use std::env;
use crate::post::{Post};
extern crate chrono;
use chrono::{Utc, TimeZone};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedditPost {
    selftext: String,
    ups: i32,
    score: i32,
    author: String,
    num_comments: i32,
    url: String,
    created: f64
}

impl RedditPost {
    pub fn to_normalized(&self) -> Post {
        return Post {
            content: self.selftext.clone(),
            reactions: self.ups + (self.ups - self.score),
            comments: self.num_comments,
            shares: 0,
            url: self.url.clone(),
            username: self.author.clone(),
            date: Utc::timestamp_millis_opt(&Utc, self.created as i64).unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
        } 
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RedditUser {
    name: String,
    created: f32,
    subreddit: RedditUserSubreddit
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RedditUserSubreddit {
    subscribers: i32,
    public_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedditUserResponse {
    data: RedditUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedditPostData {
    pub data: RedditPost
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedditListResponseData {
    pub children: Vec<RedditPostData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedditPostResponse {
    pub data: RedditListResponseData,
}

pub fn get_reddit_client(username: &str, password: &str) -> RequestClient {
    return Reddit::new(
        "Social Indexer (by /u/claudinoac)",
        &env::var("REDDIT_APP_ID").unwrap(),
        &env::var("REDDIT_APP_SECRET").unwrap(),
    ).username(username)
    .password(password)
    .login()
    .unwrap()
    .client;
}

pub fn get_reddit_user(client: &RequestClient, username: &str) -> RedditUserResponse {
    let username = username.to_string();
    let response = client.get(format!("https://reddit.com/u/{username}/about.json")).send().unwrap();
    let response_data: RedditUserResponse;
    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<RedditUserResponse>() {
                Ok(parsed) => response_data = parsed,
                Err(error) => panic!("Error while searching for user {username}! {error}"),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
    return response_data;
}


pub fn search_reddit_posts(client: &RequestClient, query: &str, limit: Option<i32>) -> RedditPostResponse {
    let limit = limit.unwrap_or(10);
    let query = query.to_string();
    let url = format!("https://reddit.com/search.json?q={query}&limit={limit}");
    let response = client.get(url).send().unwrap();
    let response_data: RedditPostResponse;
    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<RedditPostResponse>() {
                Ok(parsed) => response_data = parsed,
                Err(error) => panic!("Error while searching for user {query}! {error}"),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
    return response_data;
}


