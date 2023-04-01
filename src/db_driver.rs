use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::io::Result;

pub struct Row {}

impl Row {
    fn new() -> Row {
        Row {}
    }
}

pub struct Table {
    name: String,
    rows: Vec<Row>,
    file: File,
}

impl Table {
    pub fn new<P: AsRef<Path>>(name: &str, path: P) -> Result<Table> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let rows = vec![];

        Ok(Table {
            name: name.to_string(),
            rows,
            file,
        })
    }

    fn insert(&mut self, row: Row) -> Result<()> {
        // TODO
        Ok(())
    }

    fn search(&self, id: u32) -> Result<Option<Row>> {
        // TODO
        Ok(None)
    }

    fn save(&mut self) -> Result<()> {
        let data = &self.rows;
        self.file.set_len(0)?;
        // self.file.write_all(data.as_bytes())?;
        self.file.sync_all()?;
        Ok(())
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Table> {
        let mut file = File::open(&path)?;
        let rows = vec![];

        Ok(Table {
            name: String::new(),
            rows,
            file,
        })
    }
}
