use std::{collections::HashMap, path::PathBuf};


#[derive(Debug)]
pub enum Error {
   
}

pub type Result<T> = std::result::Result<T, Error>;

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
    pub fn get(&mut self, key: String) -> Result<String> {
       let res_string =  self.store.get(&key).cloned().unwrap();
       Ok(res_string)
    }
    ///remove a key/value pair from the
    pub fn remove(&mut self, key: String) -> bool {
        self.store.remove(&key).is_some()
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore>{
        let kv =  KvStore::new();
        return Ok(kv);
    }
}
