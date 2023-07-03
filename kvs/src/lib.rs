use std::collections::HashMap;

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
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    ///get a key/value pair from the store
    pub fn get(&mut self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
    ///remove a key/value pair from the
    pub fn remove(&mut self, key: String) -> bool {
        self.store.remove(&key).is_some()
    }
}
