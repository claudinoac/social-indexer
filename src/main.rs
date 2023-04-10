use std::io::{Result as IOResult};
use clap::Parser;
use std::path::{PathBuf};
use reqwest::Error;
use std::collections::HashMap;
mod user;
mod post;
mod reddit;
mod db_driver;
mod btree;
use reddit::{RedditPost};
use db_driver::{Row};
use btree::Node;
use chrono::prelude::*;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Parser)]
struct Cli {
    operation: String, // shell, load_data, dump_data...
    path: std::path::PathBuf,
}

fn main() -> IOResult<()> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("./btree.bin")?;

    let mut table = db_driver::Table::new("posts_table", "./posts_table.db", "post").unwrap();
    let client = reddit::get_reddit_client("ArturB32", "5A6z202@");
    let reddituser = reddit::get_reddit_user(&client, "claudinoac");
    let redditposts = reddit::search_reddit_posts(&client, "airline", Some(100));

    let mut root = Node::new(0);

    for mut reddit_post in redditposts.data.children {
        let post = RedditPost::to_normalized(&mut reddit_post.data);
        // let post_date = &post.date[0..19]; // extract the first 19 characters of the date string
        let timestamp_key = DateTime::parse_from_str(&post.date, "%Y-%m-%d %H:%M:%S");
        println!("{:#?}", timestamp_key);

        // root.insert(timestamp_key, &mut file);
        table.insert(&db_driver::Row::Post(post))?;
    }

    root.write_to_disk(&mut file)?;

    for idx in 1..10 {
        let item = table.get(idx)?;
        println!("{:}", serde_json::to_string_pretty(&item).unwrap());
    }

    Ok(())
}
