use crate::Result;

/// Key Value store trait
pub trait KvsEngine: Clone + Send + 'static {
    /// Set value with key
    fn set(&self, key: String, value: String) -> Result<()>;
    /// Get value by key
    fn get(&self, key: String) -> Result<Option<String>>;
    /// Remove key-value
    fn remove(&self, key: String) -> Result<()>;
}
pub use crate::engine::sled::SledKvsEngine;

mod kvs;
mod sled;
