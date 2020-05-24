use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

/// KvsErrorKind
#[derive(Fail, Debug, Eq, PartialEq)]
pub enum KvsErrorKind {
    #[fail(display = "IO Error")]
    IO,
    #[fail(display = "Argument Error")]
    InvalidArgument,
    #[fail(display = "Key not found Error")]
    KeyNotFound,
    #[fail(display = "Serde Error")]
    Serde,
    #[fail(display = "Index Error")]
    Index,
}

#[derive(Debug)]
pub struct KvsError {
    inner: Context<KvsErrorKind>,
}

impl KvsError {
    pub fn is_invalid_argument(&self) -> bool {
        &KvsErrorKind::KeyNotFound == self.inner.get_context()
    }
}

#[allow(dead_code)]
impl From<KvsErrorKind> for KvsError {
    fn from(kind: KvsErrorKind) -> Self {
        Self {
            inner: Context::new(kind),
        }
    }
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

impl From<std::io::Error> for KvsError {
    fn from(error: std::io::Error) -> Self {
        Self {
            inner: error.context(KvsErrorKind::IO),
        }
    }
}

impl From<serde_json::error::Error> for KvsError {
    fn from(error: serde_json::error::Error) -> Self {
        Self {
            inner: error.context(KvsErrorKind::Serde),
        }
    }
}
