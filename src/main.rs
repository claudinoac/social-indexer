use std::io::{Result};
use clap::Parser;
use std::path::{PathBuf};
mod user;
mod tweet;
mod db_driver;



#[derive(Parser)]
struct Cli {
    operation: String, // shell, load_data, dump_data...
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let table_path = PathBuf::from("./mytable.db");
    let mut table = db_driver::Table::new("mytable", &table_path)?;
    Ok(())
}
