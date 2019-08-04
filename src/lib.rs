#![deny(missing_docs)]
//! A simple key/value store.

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};
pub use messages::{Request, Response};
pub use server::KvsServer;
pub use thread_pool::{ThreadPool, NaiveThreadPool, RayonThreadPool, SharedQueueThreadPool};

mod client;
mod engines;
mod error;
mod messages;
mod server;
pub mod thread_pool;
