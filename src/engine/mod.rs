use crate::Result;

/// trait for a key/value store engine
pub trait KvsEngine {
    /*
    /// Set the value of a string key to a string
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Get the string value of a string key.
    ///
    /// If the key does not exist, return `None`.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Remove a given key.
    ///
    /// If the key does not exist, do nothing.
    fn remove(&mut self, key: String) -> Result<()>;
    */
}

pub use self::sled::SledKvsEngine;
pub use kvs::KvStore;
mod kvs;
mod sled;
