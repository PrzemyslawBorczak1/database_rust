use std::{collections::HashMap};
use crate::{errors::DatabaseResult, model::{Create, Database, DatabaseKey, Value, ValueType, commands::Insert}};



#[derive(Debug, Clone)]
pub enum Statement{
    NoStatement,
    Create(CreateSt),
    Insert(InsertSt)
}

impl Statement {
    pub fn run<K :DatabaseKey>(self, db : &mut Database<K>) -> DatabaseResult<()>{
       
        match self{
            Statement::NoStatement => {},
            Statement::Insert(i) => Insert::build_exec(db, i)?,
            Statement::Create(c) => Create::build_exec(db, c)?,
        }

        Ok(())
    }
}





#[derive(Debug, PartialEq, Clone)]
pub struct CreateSt{
    pub table_name: String,
    pub key_name: String,
    pub schema: HashMap<String,ValueType>,
}
impl CreateSt{
    pub fn new(table_name: String,  key_name: String, schema: HashMap<String,ValueType>) -> Self{
        Self { 
            table_name,
            key_name,
            schema 
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub struct InsertSt{
    pub fields: HashMap<String,Value>,
    pub table_name: String,

}

impl InsertSt{
    pub fn new(table_name : String, fields : HashMap<String,Value>) -> Self{
        Self {table_name,  fields }
    }
}





#[cfg(test)]
pub mod test{
    use std::collections::HashMap;

    use super::super::*;
    use crate::model::*;

    


    #[test]
    pub fn build_create_statement_test(){
        let query = 
                "CREATE library KEY id FIELDS id: String, title: Int, 
                CREATE lib Key i F i:float j: Bool k: Int";

        let sts = SQLParser::parse_sql(query).unwrap();
        

       match &sts[0] {
            Statement::Create(actual) => {
                let expected_fields = HashMap::from([
                    ("id".to_string(), ValueType::String),
                    ("title".to_string(), ValueType::Int),
                ]);
                let expected = CreateSt::new("library".to_string(), "id".to_string(), expected_fields);
                assert_eq!(&expected, actual, "Incorrect create statement (index 0)");
            }
            other => panic!("Expected Create at index 0, got {other:#?}"),
        }


         match &sts[1] {
            Statement::Create(actual) => {
                let expected_fields = HashMap::from([
                    ("i".to_string(), ValueType::Float),
                    ("j".to_string(), ValueType::Bool),
                    ("k".to_string(), ValueType::Int),
                ]);
                let expected = CreateSt::new("lib".to_string(), "i".to_string(), expected_fields);
                assert_eq!(&expected, actual, "Incorrect create statement (index 1)");
            }
            other => panic!("Expected Create at index 1, got {other:#?}"),
        }
    }

     #[test]
    pub fn build_insert_statement_test(){
        let query = 
                "INsert a = 1 b= 2.0 c = abc d = false into a, 
                INsert a = was b= 0.0 c = c d = true into a";

        let sts = SQLParser::parse_sql(query).unwrap();
        
         match &sts[0] {
            Statement::Insert(actual) => {
                let expected_fields = HashMap::from([
                    ("a".to_string(), Value::Int(1)),
                    ("b".to_string(), Value::Float(2.0)),
                    ("c".to_string(), Value::String("abc".to_string())),
                    ("d".to_string(), Value::Bool(false)),
                ]);
                let expected = InsertSt::new("a".to_string(), expected_fields);
                assert_eq!(&expected, actual, "Incorrect insert statement (index 0)");
            }
            other => panic!("Expected Insert at index 0, got {other:#?}"),
        }

        // Second insert
        match &sts[1] {
            Statement::Insert(actual) => {
                let expected_fields = HashMap::from([
                    ("a".to_string(), Value::String("was".to_string())),
                    ("b".to_string(), Value::Float(0.0)),
                    ("c".to_string(), Value::String("c".to_string())),
                    ("d".to_string(), Value::Bool(true)),
                ]);
                let expected = InsertSt::new("a".to_string(), expected_fields);
                assert_eq!(&expected, actual, "Incorrect insert statement (index 1)");
            }
            other => panic!("Expected Insert at index 1, got {other:#?}"),
        }
    }

}