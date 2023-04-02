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
    let table_path = PathBuf::from("./mytable.db");
    let mut table = db_driver::Table::new("mytable", &table_path)?;
    let client = reddit::get_reddit_client("---- reddit username ---- ", " ---- reddit password -----");
    let reddituser = reddit::get_reddit_user(&client, String::from("claudinoac"));
    let redditposts = reddit::search_reddit_posts(&client, "airline".to_string());
    println!("{:}", serde_json::to_string_pretty(&redditposts).unwrap());
    Ok(())
}
