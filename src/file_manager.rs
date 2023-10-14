use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
pub struct FileManager {
    collection_name: String,
}

impl FileManager {
    pub fn new(collection_name: String) -> Self {
        FileManager {
            collection_name,
        }
    }

    pub fn create_folder(&mut self, collection_name: &str) {
        match fs::create_dir(collection_name) {
            Ok(_) => {},
            Err(_) => {},
        }
    }

    pub fn read_file(&mut self, file_path: &str) -> String {
        match fs::read_to_string(format!("{}/{}", self.collection_name, String::from(file_path))) {
            Ok(data) => return data.to_string(),
            _ => return "".to_string(),
        }
    }

    pub fn write_file(&mut self, key: &str, value: &str) -> Result<(), io::Error> {
        let full_path = format!("{}/{}", self.collection_name, String::from(key));
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
        let full_path = format!("{}/{}", self.collection_name, String::from(key));
        match fs::remove_file(full_path) {
            Ok(_) => {},
            Err(error) => { eprintln!("Unable to remove file: {}", error)}
        };
    }
}
