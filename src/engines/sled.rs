use std::option::Option;
use std::path::Path;

use crate::{KvsEngine, KvsError, Result};

use sled::Db;

/// Key/value storage backend wrapper around Sled.
pub struct SledKvsEngine {
    db: Db,
}

impl SledKvsEngine {
    /// Create a new sled kv engine.
    ///
    /// # Error
    ///
    /// Return an error if sled fails to initialize.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Db::start_default(path)?;

        Ok(SledKvsEngine { db })
    }
}

impl KvsEngine for SledKvsEngine {
    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self
            .db
            .get(key)?
            .map(|v| v.as_ref().to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.set(key, value.into_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.db.del(key)?.ok_or(KvsError::KeyNotFound)?;
        self.db.flush()?;
        Ok(())
    }
}
