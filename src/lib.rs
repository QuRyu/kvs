#![deny(missing_docs)]
//! A simple key/value store.

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};
pub use messages::{Request, Response};
pub use server::KvsServer;

mod client;
mod engines;
mod error;
mod messages;
mod server;
