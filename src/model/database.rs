
use std::collections::HashMap;
use std::fmt::Debug;

use super::{Table, ValueType};



#[derive(Debug)]
pub struct Database<K: DatabaseKey>{
    tables : HashMap<String, Table<K>>,
}

impl<K : DatabaseKey> Database<K> {
    pub fn new() -> Self{
        Self {
            tables: HashMap::new() 
        }
    }

    pub fn contains_table(&self, table_name: &String) -> bool{
        self.tables.contains_key(table_name)
    }

    pub fn insert(&mut self,table_name : String,  tb : Table<K>) -> Option<Table<K>>{
        self.tables.insert(table_name, tb)
    }

    pub fn get_tables(&self) -> &HashMap<String, Table<K>>{
        &self.tables
    }
}



#[derive(Debug)]
pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>)
}


pub trait DatabaseKey : Debug + Ord + Clone {
    fn get_type() -> ValueType;

}



impl DatabaseKey for String{
    fn get_type() -> ValueType{
        ValueType::String
    }

  
}

impl DatabaseKey for i64 {
    fn get_type() -> ValueType{
        ValueType::Int
    }
}




