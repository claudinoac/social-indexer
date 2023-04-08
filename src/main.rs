use std::io::{Result as IOResult};
use clap::Parser;
use std::path::{PathBuf};
use reqwest::Error;
use std::collections::HashMap;
mod user;
mod post;
mod reddit;
mod db_driver;



#[derive(Parser)]
struct Cli {
    operation: String, // shell, load_data, dump_data...
    path: std::path::PathBuf,
}


fn main() -> IOResult<()> {
    let mut table = db_driver::Table::new("posts_table", "./posts_table.db", "post").unwrap();
    let client = reddit::get_reddit_client("---- reddit username ---- ", " ---- reddit password -----");
    let reddituser = reddit::get_reddit_user(&client, "claudinoac");
    let redditposts = reddit::search_reddit_posts(&client, "airline", Some(100));
    let reddit_post = redditposts.data.children.first().unwrap();
    let post = reddit::RedditPost::to_normalized(&reddit_post.data);
    table.insert(&db_driver::Row::Post(post));
    // println!("{:}", serde_json::to_string_pretty(&redditposts).unwrap());
    let post = reddit::RedditPost::to_normalized(&reddit_post.data);
    println!("{:}", serde_json::to_string_pretty(&post).unwrap());
    Ok(())
}
