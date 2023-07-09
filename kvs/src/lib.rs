use failure::Fail;
use std::{collections::HashMap, io, path::PathBuf};
use serde::{Serialize, Deserialize};
use serde::de::Unexpected::Str;
use std::fs::{File, OpenOptions};
use std::io::Write;

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

const  LOG_PATH: &str = "tmp.log";


#[derive(Serialize, Deserialize, Debug)]
pub struct Commend {
    op: String,
    key: String,
    value: String,
}


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
        // 序列化set 命令，插入到tmp.log中
        let commend = Commend {
            op: String::from("set"),
            key: String::from(key.clone()),
            value: String::from(value.clone())
        };
        let record = serde_json::to_string(&commend).unwrap();
        println!("{}",record);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(LOG_PATH).unwrap();
        file.write_all(record.as_bytes());
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
    ///应该是开启数据库后，从日志中读取命令回放到内存中的操作
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let kv = KvStore::new();
        return Ok(kv);
    }
}
