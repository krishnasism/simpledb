mod cache;
mod database;
mod file_manager;
fn main() {
    let mut database = Database::new();
    database.insert("krish", "mandal");
    println!("Database value:: {}", database.get("krish"));
    database.delete(String::from("krish"));
    println!("Database value:: {}", database.get("krish"));
}

pub struct Database {
    cache: cache::LRUCache,
    file_manager: file_manager::FileManager,
}
impl Database {
    pub fn new() -> Self {
        Database {
            cache: cache::LRUCache::new(1000),
            file_manager: file_manager::FileManager::new(),
        }
    }
    pub fn insert(&mut self, key: &str, value: &str) {
        self.cache.put(key, value);
        match self.file_manager.write_file(&key, &value) {
            Ok(_) => {}
            Err(error) => {eprintln!("Error {}", error)}
        }
    }

    pub fn get(&mut self, key: &str) -> String {
        let cached_value = match self.cache.get(key) {
            Some(value) => { String::from(value) }
            None => { "".to_string() }
        };
        if cached_value.len() > 0 {
            return cached_value;
        } else {
            return self.file_manager.read_file(key);
        }
    }

    pub fn update(&mut self, key: String, value: String) {
        self.insert(&key, &value)
    }

    pub fn delete(&mut self, key: String) {
        self.cache.delete(&key);
        self.file_manager.delete_file(&key);
    }
}
