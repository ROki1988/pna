use crate::error::{KvsError, KvsErrorKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

const FILE_NAME: &str = "kvs.store";
const SLINK_EXT: &str = "slink";

/// key value store
#[derive(Clone)]
pub struct KvStore {
    pub(crate) path: Arc<PathBuf>,
    pub(crate) next_pos: Arc<RwLock<usize>>,
    pub(crate) writer: Arc<Mutex<BufWriter<File>>>,
    pub(crate) index: Arc<RwLock<HashMap<String, usize>>>,
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
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let mut p: PathBuf = path.into();
        p.push(FILE_NAME);
        let file_path = p.as_path();

        if file_path.exists() & !file_path.is_file() {
            Err(KvsError::from(KvsErrorKind::IO))
        } else {
            let file = Self::open_store_file_append_mode(file_path)?;
            let log = BufReader::new(File::open(file_path)?)
                .lines()
                .map(|x| {
                    x.map_err::<KvsError, _>(Into::into).and_then(|x| {
                        serde_json::from_str(x.as_str()).map_err::<KvsError, _>(Into::into)
                    })
                })
                .collect::<Result<Vec<Command>>>()?;
            let s = Self {
                path: Arc::new(p),
                next_pos: Arc::new(RwLock::new(log.len())),
                writer: Arc::new(Mutex::new(BufWriter::new(file))),
                index: Arc::new(RwLock::new(Self::build_index(log.iter()))),
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

    fn temp_file_name_for_slink(&self) -> PathBuf {
        let mut temp = (*self.path).clone();
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

    fn ref_numbers_by_index(&self) -> Vec<usize> {
        self.index.read().unwrap().values().map(|&x| x).collect()
    }

    fn create_slink_file(&self) -> Result<usize> {
        let mut reader = BufReader::new(File::open(self.path.as_path())?).lines();
        let temp_file = self.temp_file_for_slink()?;
        let mut writer = BufWriter::new(temp_file);
        let indies = self.ref_numbers_by_index();
        let len = indies.len();

        for number in indies {
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

        *self.next_pos.write().unwrap() = len;
        Ok(())
    }
}
