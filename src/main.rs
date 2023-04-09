use std::io::{Result as IOResult};
use clap::Parser;
use std::path::{PathBuf};
use reqwest::Error;
use std::collections::HashMap;
mod user;
mod post;
mod reddit;
mod db_driver;
use reddit::{RedditPost};
use db_driver::{Row};



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
    // let reddit_post = redditposts.data.children.first().unwrap();
    for mut reddit_post in redditposts.data.children {
        let post = RedditPost::to_normalized(&mut reddit_post.data);
        table.insert(&db_driver::Row::Post(post))?;
    }
    // println!("{:}", serde_json::to_string_pretty(&redditposts).unwrap());
    for idx in 1..10 {
        let item = table.get(idx)?;
        println!("{:}", serde_json::to_string_pretty(&item).unwrap());
    }
    Ok(())
}
