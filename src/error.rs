use failure::Fail;
use std::io;

use rayon::ThreadPoolBuildError;

/// Error type for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Serialization or deserialization error
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    /// Removing non-existent key error
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
    /// Slogger initialization error
    #[fail(display = "{}", _0)]
    Sloggers(#[cause] sloggers::Error),
    /// Error response from server when request is not processed successfully.
    #[fail(display = "{}", _0)]
    ServerError(String),
    /// Sled Error
    #[fail(display = "{}", _0)]
    Sled(#[cause] sled::Error),
    /// Failure of conversion from bytes to String
    #[fail(display = "{}", _0)]
    FromUtf8Error(#[cause] std::string::FromUtf8Error),
    /// Rayon thread pool initialization error
    #[fail(display = "{}", _0)]
    RayonThreadPoolBuildError(#[cause] ThreadPoolBuildError),
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

impl From<sloggers::Error> for KvsError {
    fn from(err: sloggers::Error) -> KvsError {
        KvsError::Sloggers(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::Sled(err)
    }
}

impl From<std::string::FromUtf8Error> for KvsError {
    fn from(err: std::string::FromUtf8Error) -> KvsError {
        KvsError::FromUtf8Error(err)
    }
}

impl From<ThreadPoolBuildError> for KvsError {
    fn from(err: ThreadPoolBuildError) -> KvsError {
        KvsError::RayonThreadPoolBuildError(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
