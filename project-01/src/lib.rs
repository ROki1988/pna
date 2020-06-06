#![deny(missing_docs)]

//! use kv::KvStore;
//!
//! fn main() {
//!     let mut store = KvStore::new();
//!     store.set("key".to_owned(), "value".to_owned());
//!     assert_eq!(store.get("key".to_owned()), Some("value".to_owned()));
//! }
//!
pub use client::KvsClient;
pub use engine::KvsEngine;
pub use engine::SledKvsEngine;
pub use kv::{KvStore, Result};
pub use server::KvsServer;

mod client;
mod command;
mod engine;
mod error;
mod kv;
mod server;
