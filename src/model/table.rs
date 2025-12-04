use std::collections::{BTreeMap, HashMap};

use crate::model::DatabaseKey;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType{
    Bool,
    String, 
    Int,
    Float,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value{
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

impl Value{
    pub fn get_type(&self) -> ValueType{
        match self{
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::String(_) => ValueType::String,
        }
    }
}


#[derive(Debug)]
pub struct Table<K : DatabaseKey>{
    records: BTreeMap<K, Record>,
    schema: HashMap<String, ValueType>,
    key_name: String,
    table_name: String,
}


impl<K : DatabaseKey> Table<K>{
    pub fn new_empty(schema : HashMap<String, ValueType>, key_name: String, table_name : String) -> Self{
        Self { 
            records: BTreeMap::<K,Record>::new(),
            schema, 
            key_name, 
            table_name 
        }
    }   
    pub fn contains(&self, key: &K) -> bool{
        self.records.contains_key(key)   
    }

     pub fn add_record(&mut self, key: K, val: Record) -> Option<Record>{
        self.records.insert(key, val) 
    }

    pub fn delete_record(&mut self, key : &K) -> Option<Record>{
        self.records.remove(key)
    }


    pub fn get_records(&self) -> &BTreeMap<K, Record>{
        &self.records
    }

    pub fn get_schema(&self) -> &HashMap<String, ValueType>{
        &self.schema
    }

    pub fn get_pk(&self) -> &String{
        &self.key_name
    }

    pub fn get_tk(&self) -> &String{
        &self.table_name
    }
}



#[derive(Debug, PartialEq)]
pub struct Record{
    pub fields : HashMap<String, Value>,
}


impl Record{
    pub fn new() -> Self{
        Self { fields: HashMap::new() }
    }   

    pub fn add(&mut self, key : String, val : Value) -> Option<Value>{
        self.fields.insert(key, val)
    }
}

impl Default for Record{
    fn default() -> Self {
        Self::new()
    }
}
