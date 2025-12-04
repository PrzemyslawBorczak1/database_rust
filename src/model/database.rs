
use std::collections::HashMap;
use std::fmt::Debug;

use crate::{errors::{ExecutionResult}, parsing::CreateSt};
use super::{Table, Value, ValueType, Create};



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

impl AnyDatabase{

    pub fn contains_table(&self, table: &String) -> bool{
        match self{
            AnyDatabase::StringDatabase(db) => db.contains_table(table),
            AnyDatabase::IntDatabase(db) => db.contains_table(table),
        }
    }

    pub fn visit_create(& mut self, st: CreateSt) -> ExecutionResult<()>{
        
        match self{
            AnyDatabase::StringDatabase(db) => {
                Create::validate_create::<String>(db, st)
            },
            AnyDatabase::IntDatabase(db) => {
                Create::validate_create::<i64>(db, st)
            },
        }
    }
    

}


pub trait DatabaseKey : Debug + Ord + Clone {
    fn get_type() -> ValueType;

    fn to_type(val : &Value) -> Option<Box<&Self>>;

}



impl DatabaseKey for String{
    fn get_type() -> ValueType{
        ValueType::String
    }

    fn to_type(val : &Value) ->  Option<Box<&Self>> {
        match val {
            Value::String(s) => Some(Box::new(s)),
            _ => None,
        }
    }
}

impl DatabaseKey for i64 {
    fn get_type() -> ValueType{
        ValueType::Int
    }

    fn to_type(val : &Value) ->  Option<Box<&Self>> {
        match val {
            Value::Int(i) => Some(Box::new(i)),
            _ => None,
        }
    }
}




