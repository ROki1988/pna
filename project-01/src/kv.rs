use std::collections::HashMap;

/// key value store
pub struct KvStore(HashMap<String, String>);

impl KvStore {
    /// Return new store
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Get value by key
    pub fn get(&self, key: String) -> Option<String> {
        self.0.get(&key).map(String::to_owned)
    }

    /// Set value with key
    pub fn set(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    /// Remove key-value
    pub fn remove(&mut self, key: String)  {
        self.0.remove(&key);
    }
}
