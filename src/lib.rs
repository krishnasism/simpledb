mod cache;
mod database;
mod file_manager;
use pyo3::prelude::*;

#[pyfunction]
fn get_collection(x: String) -> Collection {
    let mut database = Collection::new(x);
    database.create();
    database
}

#[pymodule]
#[pyo3(name = "simpledb")]
fn simpledb(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_collection, m)?)?;
    Ok(())
}

#[pyclass]
pub struct Collection {
    name: String,
    cache: cache::LRUCache,
    file_manager: file_manager::FileManager,
}

#[pymethods]
impl Collection {
    #[new]
    pub fn new(name: String) -> Self {
        Collection {
            name: name.clone(),
            cache: cache::LRUCache::new(1000),
            file_manager: file_manager::FileManager::new(name.clone()),
        }
    }

    pub fn create(&mut self) {
        self.file_manager.create_folder(self.name.as_str());
    }
    pub fn insert(&mut self, key: &str, value: &str) {
        self.cache.put(key, value);
        match self.file_manager.write_file(&key, &value) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error {}", error)
            }
        }
    }

    pub fn get(&mut self, key: &str) -> String {
        let cached_value = match self.cache.get(key) {
            Some(value) => String::from(value),
            None => "".to_string(),
        };
        if cached_value.len() > 0 {
            return cached_value;
        } else {
            return self.file_manager.read_file(key);
        }
    }

    pub fn update(&mut self, key: String, value: String) {
        // No implementation for now
        self.insert(&key, &value)
    }

    pub fn delete(&mut self, key: &str) {
        self.cache.delete(&key);
        self.file_manager.delete_file(&key);
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
