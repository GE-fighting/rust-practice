use clap::Parser;

#[derive(Parser, Debug)]
pub struct KvStore{
    pub op_type:Option<String>
}

impl KvStore {
    pub fn new() -> KvStore{
        panic!()
    }

    pub fn set(&mut self, key:String, value:String){
        panic!()
    }
    pub fn get(&mut self, key:String) -> Option<String>{
        panic!()
    }

    pub fn remove(&mut self, key:String) -> bool{
        panic!()
    }
}