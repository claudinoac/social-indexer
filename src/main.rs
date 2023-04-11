use std::io::{Result as IOResult};
use std::mem;
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
use dialoguer::console::{Style};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};





#[derive(Parser)]
struct Cli {
    operation: String, // shell, load_data, dump_data...
    path: std::path::PathBuf,
}


fn main() -> IOResult<()> {
    let mut should_continue = true;
    while should_continue {
        should_continue = interactive_prompt()?;
    }
    Ok(())
}

fn search_reddit_posts() -> IOResult<()> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    let topic = Input::with_theme(&theme)
        .with_prompt("Type a topic to search about (reddit):")
        .default(String::from("reddit"))
        .interact()?;

    let num_entries : i32 = Input::with_theme(&theme)
        .with_prompt("How many entries do you want to fetch? (100):")
        .default(100)
        .interact()?;

    let client = reddit::get_reddit_client("---- reddit username ---- ", " ---- reddit password -----");
    let reddituser = reddit::get_reddit_user(&client, "claudinoac");
    let redditposts = reddit::search_reddit_posts(&client, &topic, Some(num_entries));
    let mut table = db_driver::Table::new("posts_table", "./posts_table.db", "post").unwrap();
    for mut reddit_post in redditposts.data.children {
        let post = RedditPost::to_normalized(&mut reddit_post.data);
        // let reddituser = reddit::get_reddit_user(&client, &post.username);
        println!("{:}", serde_json::to_string_pretty(&post).unwrap());
        table.insert(&db_driver::Row::Post(post))?;
    }
    Ok(())
}

fn list_all_users() {
    
}


fn list_all_posts() -> IOResult<()> {
    let mut table = db_driver::Table::new("posts_table", "./posts_table.db", "post").unwrap();
    let entries = table.all()?;
    let size = entries.len();
    for item in entries {
        println!("{:}", serde_json::to_string_pretty(&item).unwrap());
    }
    Ok(())
}


fn interactive_prompt() -> IOResult<bool> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to social indexer");

    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        return Ok(false);
    }
    
    let method_options = vec![
        "List all posts",
        "List all users",
        "Search and store reddit posts",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose one of the following options.")
        .default(0)
        .items(&method_options[..])
        .interact_opt()?;

    match selection {
        Some(0) => {
            list_all_posts();
        },
        Some(1) => {
            list_all_users();
        },
        Some(2) => {
            search_reddit_posts();
        },
        _ => {}
    }

    Ok(true)
}
