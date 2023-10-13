use std::collections::{HashMap, LinkedList};
pub struct LRUCache {
    cache: HashMap<String, (String, LinkedList<String>)>,
    lru_list: LinkedList<String>,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            cache: HashMap::new(),
            lru_list: LinkedList::new(),
            capacity,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&str> {
        if let Some((value, node)) = self.cache.get_mut(key) {
            // Move the accessed key to the front of the list (most recently used)
            let last_element = node.back();
            match last_element {
                Some(_) => {
                    self.lru_list.push_front(node.pop_back().unwrap());
                    node.push_front(key.to_string());
                    Some(value)
                }
                _ => {
                    node.push_front(key.to_string());
                    Some(value)
                }
            }
        } else {
            None
        }
    }

    pub fn put(&mut self, key: &str, value: &str) {
        let _key = String::from(key);
        let _value = String::from(value);

        if let Some((_, node)) = self.cache.get_mut(&_key) {
            let mut cloned_node = node.clone();
            cloned_node.push_front(_key.clone());
            self.lru_list.push_front(cloned_node.pop_back().unwrap());
            self.cache.insert(_key, (_value, cloned_node));
        } else {
            let mut new_node = LinkedList::new();
            new_node.push_front(_key.clone());
            self.lru_list.push_front(_key.clone());
            self.cache.insert(_key, (_value, new_node));
        }
        if self.cache.len() > self.capacity {
            if let Some(oldest_key) = self.lru_list.pop_back() {
                self.cache.remove(&oldest_key);
            }
        }
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        if let Some((value, node)) = self.cache.remove(key) {
            let cloned_node: LinkedList<String> = node.into_iter().filter(|k| k != key).collect();
            self.lru_list = cloned_node;
            Some(value)
        } else {
            None
        }
    }
}
