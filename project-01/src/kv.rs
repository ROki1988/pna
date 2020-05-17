use failure::{Backtrace, Context, Fail};

use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt::Display;
use std::fmt;

/// key value store
pub struct KvStore(HashMap<String, String>);

/// KvsErrorKind
#[derive(Fail, Debug)]
pub enum KvsErrorKind {
    #[fail(display = "IO Error")]
    IO,
}


#[derive(Debug)]
pub struct KvsError {
    inner: Context<KvsErrorKind>,
}

#[allow(dead_code)]
impl Fail for KvsError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

#[allow(dead_code)]
impl Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

/// Result alias
pub type Result<T> = std::result::Result<T, KvsError>;

impl KvStore {
    /// Return new store
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(Self(HashMap::new()))
    }

    /// Get value by key
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.0.get(&key).map(String::to_owned))
    }

    /// Set value with key
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.0.insert(key, value);
        Ok(())
    }

    /// Remove key-value
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.0.remove(&key);
        Ok(())
    }
}
