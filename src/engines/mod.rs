//! This module provides various key value storage engines.
pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;
use crate::Result;

mod kvs;
mod sled;

/// Trait for a key value storage engine.
pub trait KvsEngine: Clone + Send + 'static {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Get the string value of a given string key.
    ///
    /// Return `None` if the given key does not exits.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key not found.
    fn remove(&self, key: String) -> Result<()>;
}
