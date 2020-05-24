#![deny(missing_docs)]

//! use kv::KvStore;
//!
//! fn main() {
//!     let mut store = KvStore::new();
//!     store.set("key".to_owned(), "value".to_owned());
//!     assert_eq!(store.get("key".to_owned()), Some("value".to_owned()));
//! }
//!
pub use kv::{KvStore, Result};

mod error;
mod kv;
