#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use kv::{KvStore, KvsEngine};
pub use messages::{Request, Response};

mod error;
mod kv;
mod messages;
