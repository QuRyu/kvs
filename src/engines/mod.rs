pub use self::kv::KvStore;
pub use self::sled::SledKvsEngine;
use crate::Result;

mod kv;
mod sled;

/// Define the storage interface for a key/value engine.
pub trait KvsEngine: Clone + Send + 'static {
    /// Set the value of a string key to a value.
    ///
    /// # Error
    ///
    /// Return an error if the value is not written successfully.
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Get the string value of a string key.
    /// If the key does not exist, return `None`.
    ///
    /// # Error
    ///
    /// Return an error if the value is not read successfully.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Remove a string key.
    ///
    /// # Error
    ///
    /// Return an error if the key is not present or
    /// the value is not read successfully.
    fn remove(&self, key: String) -> Result<()>;
}
