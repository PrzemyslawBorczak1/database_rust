
use std::collections::HashMap;
use std::fmt::Debug;

use crate::model::Value;

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

    pub fn get_table<'a>(&'a mut self, table_name: &String) ->  Option<&'a mut Table<K>>{
        self.tables.get_mut(table_name)
    }
}



#[derive(Debug)]
pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>)
}


pub trait DatabaseKey : Debug + Ord + Clone {
    fn get_type() -> ValueType;

    fn as_key(val : Value) -> Option<Self>;
}



impl DatabaseKey for String{
    fn get_type() -> ValueType{
        ValueType::String
    }

    fn as_key(val : Value) -> Option<Self>{
        
        match val {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

  
}

impl DatabaseKey for i64 {
    fn get_type() -> ValueType{
        ValueType::Int
    }

     fn as_key(val : Value) -> Option<Self>{
        
        match val {
            Value::Int(s) => Some(s),
            _ => None,
        }
    }
}




