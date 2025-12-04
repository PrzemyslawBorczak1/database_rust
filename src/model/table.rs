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


#[derive(Debug)]
pub struct Record{
    pub fields : HashMap<String, Value>,
}

