#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use engines::{KvStore, KvsEngine};
pub use messages::{Request, Response};
pub use client::KvsClient;

mod error;
mod engines;
mod messages;
mod client;
