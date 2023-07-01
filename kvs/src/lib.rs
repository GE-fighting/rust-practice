use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore{
   store: HashMap<String,String>,
}

impl KvStore {
    pub fn new() -> KvStore{
        let mut mapStore: HashMap<String, String> = HashMap::new();
        KvStore{
            store: mapStore,
        }
    }

    pub fn set(&mut self, key:String, value:String){
        self.store.insert(key, value);
    }
    pub fn get(&mut self, key:String) -> Option<String>{
        self.store.get(&key).cloned()
    }

    pub fn remove(&mut self, key:String) -> bool{
        self.store.remove(&key).is_some()
    }
}