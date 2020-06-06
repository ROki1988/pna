use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;
use strum::ParseError;
use strum_macros::EnumString;

/// KvsErrorKind
#[derive(Fail, Debug, EnumString, Eq, PartialEq)]
pub enum KvsErrorKind {
    #[fail(display = "IO")]
    IO,
    #[fail(display = "InvalidArgument")]
    InvalidArgument,
    #[fail(display = "KeyNotFound")]
    KeyNotFound,
    #[fail(display = "Serde")]
    Serde,
    #[fail(display = "UnknownCommand")]
    UnknownCommand(String),
    #[fail(display = "WrongFormat")]
    WrongFormat(String),
    #[fail(display = "Index")]
    Index,
    #[fail(display = "Engine")]
    Engine,
    #[fail(display = "Encoding")]
    Encoding,
    #[fail(display = "Parse")]
    Parse,
}

#[derive(Debug)]
pub struct KvsError {
    inner: Context<KvsErrorKind>,
}

impl KvsError {
    pub fn is_key_not_found(&self) -> bool {
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

impl From<sled::Error> for KvsError {
    fn from(error: sled::Error) -> Self {
        Self {
            inner: error.context(KvsErrorKind::Engine),
        }
    }
}

impl From<bstr::Utf8Error> for KvsError {
    fn from(error: bstr::Utf8Error) -> Self {
        Self {
            inner: error.context(KvsErrorKind::Encoding),
        }
    }
}

impl From<strum::ParseError> for KvsError {
    fn from(error: ParseError) -> Self {
        Self {
            inner: error.context(KvsErrorKind::Parse),
        }
    }
}
