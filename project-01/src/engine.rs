use crate::Result;

/// Key Value store trait
pub trait KvsEngine {
    /// Set value with key
    fn set(&mut self, key: String, value: String) -> Result<()>;
    /// Get value by key
    fn get(&mut self, key: String) -> Result<Option<String>>;
    /// Remove key-value
    fn remove(&mut self, key: String) -> Result<()>;
}
pub use crate::engine::sled::SledKvsEngine;

mod kvs;
mod sled;
