use failure::Fail;
use std::{collections::HashMap, io, path::PathBuf};
use serde::{Serialize, Deserialize};
#[derive(Debug, Fail)]
pub enum KvError {
    /// Io error
    #[fail(display = "io error occurred.")]
    IoError(#[cause] io::Error),

    ///serde  error
    #[fail(display = "serde error occurred.")]
    SerdeErr(#[cause] serde_json::Error)

}

pub type Result<T> = std::result::Result<T, KvError>;






#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Create a new mem store
    pub fn new() -> KvStore {
        let mut mapStore: HashMap<String, String> = HashMap::new();
        KvStore { store: mapStore }
    }
    ///set a key/value pair in the store
    pub fn set(&mut self, key: String, value: String) -> Result<bool> {
        self.store.insert(key, value);
        Ok(true)
    }
    ///get a key/value pair from the store
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let res_string = self.store.get(&key).cloned().unwrap();
        Ok(Some(res_string))
    }
    ///remove a key/value pair from the
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key).is_some();
        Ok(())
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let kv = KvStore::new();
        return Ok(kv);
    }
}
