use crate::Result;
use crate::{KvsEngine, KvsError};
use std::path::PathBuf;

/// The `SledKvsEngine` is used to store Key/Value pairs based on `sled`.
/// Example:
///
/// ```rust
/// # use kvs::{SledKvsEngine, Result};
/// use std::env::current_dir;
/// use kvs::KvsEngine;
/// fn main() -> Result<()> {
/// let mut store = SledKvsEngine::open(current_dir()?)?;
/// store.set("key".to_string(), "value".to_string())?;
/// store.set("key1".to_string(), "value1".to_string())?;
/// store.remove("key1".to_string())?;
/// assert_eq!(store.get("key".to_string())?, Some("value".to_string()));
/// assert_eq!(store.get("key1".to_string())?, None);
/// Ok(())
/// }
///```
pub struct SledKvsEngine {
    db: sled::Db,
}

impl SledKvsEngine {
    /// create a new `SledKvsEngine` engine
    pub fn open(path: impl Into<PathBuf>) -> Result<SledKvsEngine> {
        Ok(SledKvsEngine {
            db: sled::open(path.into())?,
        })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_str(), value.as_str())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let value = self.db.get(key.as_str())?.map(|ivec| ivec.to_vec());
        if let Some(value) = value {
            let s = String::from_utf8(value)?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let old_value = self.db.remove(key.as_str())?;
        if old_value.is_some() {
            self.db.flush()?;
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }
}
