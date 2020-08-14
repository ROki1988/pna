use crate::engine::KvsEngine;
use crate::error::{KvsError, KvsErrorKind};
use crate::kv::{Command, KvStore};
use crate::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

impl KvsEngine for KvStore {
    fn set(&self, key: String, value: String) -> Result<()> {
        let command = Command::Set((key.clone(), value));
        let s: String = serde_json::to_string(&command)?;
        let mut w = self.writer.lock().expect("set: cant write");
        w.write_fmt(format_args!("{}\n", s))?;
        w.flush()?;
        let mut pos = self.next_pos.write().unwrap();
        self.index
            .write()
            .expect("set: cant insert")
            .insert(key, *pos);
        *pos += 1;

        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        if let Some(&i) = self.index.read().unwrap().get(key.as_str()) {
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

    fn remove(&self, key: String) -> Result<()> {
        let _v = self
            .get(key.clone())?
            .ok_or_else(|| KvsError::from(KvsErrorKind::KeyNotFound))?;
        let command = Command::Rm(key.clone());
        let s: String = serde_json::to_string(&command)?;
        let mut w = self.writer.lock().expect("remove: cant write");
        w.write_fmt(format_args!("{}\n", s))?;
        w.flush()?;
        self.index
            .write()
            .expect("remove: cant remove")
            .remove(key.as_str());
        *self.next_pos.write().expect("remove: cant increment") += 1;
        Ok(())
    }
}
