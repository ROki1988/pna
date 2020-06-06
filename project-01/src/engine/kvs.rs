use crate::engine::KvsEngine;
use crate::error::{KvsError, KvsErrorKind};
use crate::kv::{Command, KvStore};
use crate::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set((key.clone(), value));
        let s: String = serde_json::to_string(&command)?;
        self.file.write_fmt(format_args!("{}\n", s))?;
        self.index.insert(key, self.next_pos);
        self.next_pos += 1;

        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(&i) = self.index.get(key.as_str()) {
            let reader = BufReader::new(File::open(self.path.as_path())?);
            let s = reader
                .lines()
                .nth(i)
                .ok_or_else(|| KvsError::from(KvsErrorKind::Index))?;
            let command: Command = serde_json::from_str(s?.as_str())?;
            match command {
                Command::Set(s) => Ok(Some(s.1)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let _v = self
            .get(key.clone())?
            .ok_or_else(|| KvsError::from(KvsErrorKind::KeyNotFound))?;
        let command = Command::Rm(key.clone());
        let s: String = serde_json::to_string(&command)?;
        self.file.write_fmt(format_args!("{}\n", s))?;
        self.index.remove(key.as_str());
        self.next_pos += 1;
        Ok(())
    }
}
