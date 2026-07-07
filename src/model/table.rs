use std::collections::{BTreeMap, HashMap};

use crate::model::DatabaseKey;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType{
    Bool,
    String,
    Int,
    Float,
}

impl ValueType {
    /// Nazwa typu tak, jak wystepuje w skladni CREATE ... FIELDS.
    pub fn to_query_name(&self) -> &'static str {
        match self {
            ValueType::Bool => "Bool",
            ValueType::String => "String",
            ValueType::Int => "Int",
            ValueType::Float => "Float",
        }
    }
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

    /// Tekstowa reprezentacja wartosci do wyswietlania w wynikach SELECT.
    pub fn to_display(&self) -> String {
        match self {
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
        }
    }

    /// Reprezentacja wartosci w skladni zapytania (dla SAVE_AS / to_query).
    /// W odroznieniu od `to_display` gwarantuje, ze Float ma kropke dziesietna,
    /// bo inaczej reparsowalby sie jako Int (np. 3.0 -> "3" -> Int).
    pub fn to_query_value(&self) -> String {
        match self {
            Value::Float(f) if f.fract() == 0.0 && f.is_finite() => format!("{:.1}", f),
            other => other.to_display(),
        }
    }

    /// Porownanie dwoch wartosci na potrzeby ORDER_BY.
    /// Rozne typy uznajemy za rowne (nieporownywalne).
    pub fn compare(&self, other: &Value) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Value::String(a), Value::String(b)) => a.cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
            _ => Ordering::Equal,
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
