use failure::Fail;
use std::process::exit;
use std::{collections::HashMap, io, path::PathBuf};
use serde::{Serialize, Deserialize};
use serde::de::Unexpected::Str;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

#[derive(Debug, Fail)]
pub enum KvError {
    /// Io error
    #[fail(display = "io error occurred.")]
    IoError(#[cause] io::Error),

    ///serde  error
    #[fail(display = "serde error occurred.")]
    SerdeErr(#[cause] serde_json::Error)

}

impl From<serde_json::Error> for KvError {
    fn from(error: serde_json::Error) -> Self {
        KvError::SerdeErr(error)
    }
}

impl From<io::Error>for KvError {
    fn from(value: Error) -> Self {
        KvError::IoError(value)
    }
}


pub type Result<T> = std::result::Result<T, KvError>;

const  LOG_PATH: &str = "tmp.log";

#[derive(Serialize, Deserialize, Debug)]
pub enum Commend{
    Set{key: String,value: String},
    Remove{key: String}
}

impl Commend {
    fn set(key: String,value:String) -> Commend{
        Commend::Set {key,value}
    }
    fn remove(key:String) -> Commend{
        Commend::Remove {key}
    }
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
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // 序列化set 命令，插入到tmp.log中
        let commend = Commend::set(key.clone(),value.clone());
        let record = serde_json::to_string(&commend)?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(LOG_PATH)?;
        file.write_all(record.as_bytes())?;
        self.store.insert(key, value);
        Ok(())
    }
    ///get a key/value pair from the store
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let res_string = self.store.get(&key).cloned().unwrap();
        Ok(Some(res_string))
    }
    ///remove a key/value pair from the
    pub fn remove(&mut self, key: String) -> Result<()> {
        let ok =  self.store.contains_key(&key);
        if !ok{
            println!("Key not found");
            exit(1);
        }
        let commend = Commend::remove(key.clone());
        let record = serde_json::to_string(&commend)?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(LOG_PATH)?;
        file.write_all(record.as_bytes())?;
        self.store.remove(&key).is_some();
        Ok(())
    }
    ///应该是开启数据库后，从日志中读取命令回放到内存中的操作
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {

        let kv = KvStore::new();
        return Ok(kv);
    }
}
