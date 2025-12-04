
use crate::{ errors::{ExecutionErr, ExecutionResult}, model::{Database, DatabaseKey, Table, ValueType}};
use super::super::{AnyDatabase};
use crate::parsing::CreateSt;

use crate::errors::{DatabaseErr, DatabaseResult, StatementErr};




#[derive(Debug)]
pub struct Create<'a>{
    db: &'a mut AnyDatabase,
    st: CreateSt,
}


impl<'a> Create<'a> {
    pub fn new(db: &'a mut AnyDatabase, st: CreateSt) -> Self{
        Self { db, st }
    }


    pub fn execute(self) -> DatabaseResult<()>{
        match self.db.visit_create(self.st){
            Err(e) => Err(DatabaseErr::ExecutionError{
                error : e,
                statement: StatementErr::Create
            }),
            Ok(_) => Ok(()), 
        }
    }


    pub fn validate_create<K : DatabaseKey>(db: &'a mut Database<K>, st: CreateSt) -> ExecutionResult<()>{
        if db.contains_table(&st.table_name){
            return Err(ExecutionErr::TableAlreadyPresent(st.table_name.clone()))
        };

        let pk = st.schema.get(&st.key_name);
        let vt = match pk {
            None => return Err(ExecutionErr::NoPK(st.key_name.clone())),
            Some(x) => x,
        };

        if vt != &K::get_type(){
            return Err(ExecutionErr::WrongPKType { expected: K::get_type(), got: vt.clone() });
        }

        let tb: Table<K> = Table::new_empty(st.schema, st.key_name, st.table_name.clone());
        if  db.insert(st.table_name.clone(), tb).is_some(){
            return Err(ExecutionErr::TableAlreadyPresent(st.table_name.clone()))
        }

        Ok(())
    }
    
}





#[cfg(test)]
pub mod test{
    use std::collections::HashMap;

    use super::*;
    use crate::parsing::*;
    #[test]
    pub fn test() {
        let query =
            "CREATE a KEY a FIELDS a: String, c: Int, 
            CREATE b KEY a FIELDS a: String, b: Float c: Bool";

        let sts = SQLParser::parse_sql(query).unwrap();
        let mut db = AnyDatabase::StringDatabase(Database::new());

        if let Statement::Create(st) = sts[0].clone() {
            Create::new(&mut db, st).execute().unwrap()
        } else {
            panic!();
        }

        if let Statement::Create(st) = sts[1].clone() {
            Create::new(&mut db, st).execute().unwrap()
        } else {
            panic!();
        }

        match &db {
            AnyDatabase::StringDatabase(inner) => {
                let tables = inner.get_tables();

                let tb = tables.get("a").unwrap();
                assert!(tb.get_records().is_empty());
                let exp = HashMap::from([
                    ("a".to_string(), ValueType::String),
                    ("c".to_string(), ValueType::Int),
                ]);
                assert_eq!(tb.get_schema(), &exp);
                assert_eq!(tb.get_pk(), "a");

                let tb = tables.get("b").unwrap();
                assert!(tb.get_records().is_empty());
                let exp = HashMap::from([
                    ("a".to_string(), ValueType::String),
                    ("b".to_string(), ValueType::Float),
                    ("c".to_string(), ValueType::Bool),
                ]);
                assert_eq!(tb.get_schema(), &exp);
                assert_eq!(tb.get_pk(), "a");
            }
            AnyDatabase::IntDatabase(_) => {
                panic!();
            }
        }
    }
}