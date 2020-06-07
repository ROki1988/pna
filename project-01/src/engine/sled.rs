use crate::error::{KvsError, KvsErrorKind};
use crate::{KvsEngine, Result};
use bstr::ByteSlice;
use sled;
use std::path::PathBuf;

/// Key value store by sled
pub struct SledKvsEngine(sled::Db);

const FILE_NAME: &str = "sled.store";

impl SledKvsEngine {
    /// open kvs
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let mut p: PathBuf = path.into();
        p.push(FILE_NAME);
        let inner = sled::open(p)?;
        Ok(Self(inner))
    }
}

impl Drop for SledKvsEngine {
    fn drop(&mut self) {
        self.0.flush().unwrap();
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.0.insert(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        self.0
            .get(key)?
            .map(|x| x.to_str().map_err(Into::into).map(str::to_string))
            .transpose()
    }

    fn remove(&mut self, key: String) -> Result<()> {
        if !self.0.contains_key(key.as_str())? {
            return Err(KvsError::from(KvsErrorKind::KeyNotFound));
        }
        self.0.remove(key)?;
        Ok(())
    }
}
