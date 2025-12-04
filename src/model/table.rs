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
