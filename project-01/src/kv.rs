use crate::error::{KvsError, KvsErrorKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{PathBuf, Path};

const FILE_NAME: &str = "kvs.store";
const SLINK_EXT: &str = "slink";

/// key value store
pub struct KvStore {
    path: PathBuf,
    len: usize,
    file: File,
    index: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set((String, String)),
    Rm(String),
}

/// Result alias
pub type Result<T> = std::result::Result<T, KvsError>;

impl KvStore {
    /// Return new store
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut p: PathBuf = path.into();
        p.push(FILE_NAME);
        let path = p.as_path();

        if path.exists() & !path.is_file() {
            Err(KvsError::from(KvsErrorKind::IO))
        } else {
            let file = Self::open_store_file_append_mode(path)?;
            let log = BufReader::new(File::open(path)?)
                .lines()
                .map(|x| {
                    x.map_err::<KvsError, _>(Into::into).and_then(|x| {
                        serde_json::from_str(x.as_str()).map_err::<KvsError, _>(Into::into)
                    })
                })
                .collect::<Result<Vec<Command>>>()?;
            let s = Self {
                path: p,
                len: log.len(),
                file,
                index: Self::build_index(log.iter()),
            };
            Ok(s)
        }
    }

    fn open_store_file_append_mode(path: &Path) -> Result<File> {
        OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .create(true)
            .open(path)
            .map_err(Into::into)
    }

    fn build_index<'a>(iter: impl Iterator<Item = &'a Command>) -> HashMap<String, usize> {
        iter.enumerate().fold(HashMap::new(), |mut acc, x| {
            match x.1 {
                Command::Set(s) => {
                    acc.insert(s.0.clone(), x.0);
                }
                Command::Rm(r) => {
                    acc.remove(r.as_str());
                }
            }
            acc
        })
    }

    /// Get value by key
    pub fn get(&self, key: String) -> Result<Option<String>> {
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

    /// Set value with key
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set((key.clone(), value));
        let s: String = serde_json::to_string(&command)?;
        self.file
            .write_fmt(format_args!("{}\n", s))?;
        self.len += 1;
        self.index.insert(key, self.len - 1);

        Ok(())
    }

    /// Remove key-value
    pub fn remove(&mut self, key: String) -> Result<()> {
        let _v = self
            .get(key.clone())?
            .ok_or_else(|| KvsError::from(KvsErrorKind::KeyNotFound))?;
        let command = Command::Rm(key.clone());
        let s: String = serde_json::to_string(&command)?;
        self.file
            .write_fmt(format_args!("{}\n", s))?;
        self.len += 1;
        self.index.remove(key.as_str());
        Ok(())
    }

    fn temp_file_name_for_slink(&self) -> PathBuf {
        let mut temp = self.path.clone();
        temp.set_extension(SLINK_EXT);
        temp
    }

    fn temp_file_for_slink(&self) -> Result<File> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.temp_file_name_for_slink())
            .map_err::<KvsError, _>(Into::into)
    }

    fn ref_numbers_by_index(&self) -> Vec<&usize> {
        let mut indies: Vec<&usize> = self.index.values().collect();
        indies.sort();
        indies
    }

    fn create_slink_file(&self) -> Result<usize> {
        let mut reader = BufReader::new(File::open(self.path.as_path())?).lines();
        let temp_file = self.temp_file_for_slink()?;
        let mut writer = BufWriter::new(temp_file);
        let indies = self.ref_numbers_by_index();
        let len = indies.len();

        for &number in indies {
            let l = reader
                .nth(number)
                .ok_or_else(|| KvsError::from(KvsErrorKind::Index))?;
            println!("{}", number);
            l.and_then(|x| writeln!(writer, "{}", x))?;
        }

        Ok(len)
    }

    /// Slink log file
    pub fn slink(&mut self) -> Result<()> {
        let len = self.create_slink_file()?;

        let file_path = self.temp_file_name_for_slink();
        std::fs::copy(file_path.as_path(), self.path.as_path())?;
        std::fs::remove_file(file_path.as_path())?;

        self.len = len - 1;
        Ok(())
    }
}
