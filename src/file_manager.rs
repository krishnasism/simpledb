use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
pub struct FileManager {}

impl FileManager {
    pub fn new() -> Self {
        FileManager {}
    }
    pub fn read_file(&mut self, file_path: &str) -> String {
        match fs::read_to_string(format!("documents/{}", String::from(file_path))) {
            Ok(data) => return data.to_string(),
            _ => return "".to_string(),
        }
    }

    pub fn write_file(&mut self, key: &str, value: &str) -> Result<(), io::Error> {
        let full_path = format!("documents/{}", String::from(key));
        let path_exists = Path::new(&full_path).exists();
        if path_exists {
            println!("Record exists, overwriting");
        }
        let mut file = match File::create(full_path) {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        match file.write_all(value.as_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => {
                eprintln!("Error in writing to file {}", value);
                Err(error)
            }
        }
    }
    
    pub fn delete_file(&mut self, key: &str) {
        let full_path = format!("documents/{}", String::from(key));
        match fs::remove_file(full_path) {
            Ok(_) => {},
            Err(error) => { eprintln!("Unable to remove file: {}", error)}
        };
    }
}
