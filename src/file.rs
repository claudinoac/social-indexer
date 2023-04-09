use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use rand::{Rng, thread_rng};

// Define a struct for our entries
struct Entry {
    id: u32,
    name: String,
    description: String,
}

impl Entry {
    // Convert our entry to a byte buffer for writing to disk
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec<u8>::new();
        buffer.write_u32::<byteorder::LittleEndian>(self.id).unwrap();
        buffer.write_u32::<byteorder::LittleEndian>(self.name.len() as u32).unwrap();
        buffer.write_all(self.name.as_bytes()).unwrap();
        buffer.write_u32::<byteorder::LittleEndian>(self.description.len() as u32).unwrap();
        buffer.write_all(self.description.as_bytes()).unwrap();
        buffer
    }

    // Convert a byte buffer from disk into an entry
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut cursor = io::Cursor::new(bytes);
        let id = cursor.read_u32::<byteorder::LittleEndian>().unwrap();
        let name_len = cursor.read_u32::<byteorder::LittleEndian>().unwrap();
        let mut name_buffer = vec![0; name_len as usize];
        cursor.read_exact(&mut name_buffer).unwrap();
        let name = String::from_utf8(name_buffer).unwrap();
        let desc_len = cursor.read_u32::<byteorder::LittleEndian>().unwrap();
        let mut desc_buffer = vec![0; desc_len as usize];
        cursor.read_exact(&mut desc_buffer).unwrap();
        let description = String::from_utf8(desc_buffer).unwrap();
        Self { id, name, description }
    }
}

// Define our CRUD operations
fn create_entry(entries_file: &mut File, name: &str, description: &str) -> io::Result<Entry> {
    let id = thread_rng().gen::<u32>();
    let entry = Entry { id, name: name.to_string(), description: description.to_string() };
    let bytes = entry.to_bytes();
    entries_file.write_all(&bytes)?;
    Ok(entry)
}

fn read_entry(entries_file: &mut File, id: u32) -> io::Result<Option<Entry>> {
    entries_file.seek(io::SeekFrom::Start(0))?;
    let mut buffer = Vec::new();
    let mut found = false;
    while let Ok(bytes_read) = entries_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let entry = Entry::from_bytes(&buffer);
        if entry.id == id {
            found = true;
            break;
        }
        buffer.clear();
    }
    if found {
        Ok(Some(entry))
    } else {
        Ok(None)
    }
}

fn update_entry(entries_file: &mut File, id: u32, name: &str, description: &str) -> io::Result<Option<Entry>> {
    entries_file.seek(io::SeekFrom::Start(0))?;
    let mut buffer = Vec::new();
    let mut found = false;
    while let Ok(bytes_read) = entries_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let entry = Entry::from_bytes(&buffer);
        if entry.id == id {
            let updated_entry = Entry { id, name: name.to_string(), description: description.to_string() };
            let bytes = updated_entry.to_bytes();
            entries_file.seek
