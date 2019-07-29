#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use kv::{KvStore, KvsEngine};

mod error;
mod kv;
