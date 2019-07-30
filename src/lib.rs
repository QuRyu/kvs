#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use engines::{KvStore, KvsEngine};
pub use messages::{Request, Response};

mod error;
mod engines;
mod messages;
