use std::collections::HashMap;

pub struct KvStore(HashMap<String, String>);

impl KvStore {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.0.get(&key).map(String::to_owned)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    pub fn remove(&mut self, key: String)  {
        self.0.remove(&key);
    }
}
