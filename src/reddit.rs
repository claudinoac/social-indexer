use roux::{Reddit};
use reqwest::blocking::{Client as RequestClient};
use serde::{Deserialize, Serialize};
use std::env;


#[derive(Serialize, Deserialize, Debug)]
pub struct RedditPost {
    selftext: String,
    ups: i32,
    score: i32,
    author: String,
    num_comments: i32,
    url: String,
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
    data: RedditPost
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedditListResponseData {
    children: Vec<RedditPostData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedditPostResponse {
    data: RedditListResponseData,
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

pub fn get_reddit_user(client: &RequestClient, username: String) -> RedditUserResponse {
    let response = client.get("https://reddit.com/u/claudinoac/about.json").send().unwrap();
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


pub fn search_reddit_posts(client: &RequestClient, query: String) -> RedditPostResponse {
    let url = format!("https://reddit.com/search.json?q={query}&limit=10");
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


