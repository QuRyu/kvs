#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use messages::{Request, Response};
pub use client::KvsClient;
pub use server::KvsServer;

mod error;
mod engines;
mod messages;
mod client;
mod server;
