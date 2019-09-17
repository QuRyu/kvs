use std::option::Option;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::{KvsEngine, KvsError, Result};

use sled::Db;

/// Key/value storage backend wrapper around Sled.
pub struct SledKvsEngine {
    data: Arc<Mutex<SledKvsEngineData>>,
}

impl SledKvsEngine {
    /// Create a new sled kv engine.
    ///
    /// # Error
    ///
    /// Return an error if sled fails to initialize.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = SledKvsEngineData::new(path)?;

        Ok(SledKvsEngine {
            data: Arc::new(Mutex::new(data)),
        })
    }
}

impl KvsEngine for SledKvsEngine {
    fn get(&self, key: String) -> Result<Option<String>> {
        let mut data = self.data.lock().unwrap();

        data.get(key)
    }

    fn set(&self, key: String, value: String) -> Result<()> {
        let mut data = self.data.lock().unwrap();

        data.set(key, value)
    }

    fn remove(&self, key: String) -> Result<()> {
        let mut data = self.data.lock().unwrap();

        data.remove(key)
    }
}

impl Clone for SledKvsEngine {
    fn clone(&self) -> Self {
        SledKvsEngine {
            data: self.data.clone(),
        }
    }
}

struct SledKvsEngineData {
    db: Db,
}

impl SledKvsEngineData {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Db::start_default(path)?;

        Ok(SledKvsEngineData { db })
    }

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
