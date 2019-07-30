use crate::{Result};

/// Define the storage interface for a key/value engine. 
pub trait KvsEngine { 
    /// Set the value of a string key to a value. 
    ///
    /// # Error 
    ///
    /// Return an error if the value is not written successfully. 
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Get the string value of a string key. 
    /// If the key does not exist, return `None`. 
    ///
    /// # Error 
    ///
    /// Return an error if the value is not read successfully. 
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Remove a string key.
    ///
    /// # Error 
    ///
    /// Return an error if the key is not present or 
    /// the value is not read successfully.
    fn remove(&mut self, key: String) -> Result<()>;
}
