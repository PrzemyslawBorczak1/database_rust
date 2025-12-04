use crate::errors::{ExecutionErr, ExecutionResult, DatabaseErr, DatabaseResult, StatementErr};
use crate::model::{Database, DatabaseKey, Table};
use crate::parsing::CreateSt;
use super::Command;




#[derive(Debug)]
pub struct Create<'a, K : DatabaseKey>{
    db: &'a mut Database<K>,
    st: CreateSt,
}


impl<'a, K : DatabaseKey> Create<'a, K> {
    pub fn build_exec(db: &'a mut Database<K>, st: CreateSt) -> DatabaseResult<()>{
        let c = Self::new(db,st);
        match c.execute(){
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseErr::ExecutionError { error: e, statement: StatementErr::Create })
        }
    }

    pub fn new(db: &'a mut Database<K>, st: CreateSt) -> Self{
        Self { db, st }
    }
}


impl<'a, K : DatabaseKey> Command for Create<'a, K>{
    fn execute(self) -> ExecutionResult<()>{
         if self.db.contains_table(&self.st.table_name){
            return Err(ExecutionErr::TableAlreadyPresent(self.st.table_name.clone()))
        };

        let pk = self.st.schema.get(&self.st.key_name);
        let vt = match pk {
            None => return Err(ExecutionErr::NoPK(self.st.key_name.clone())),
            Some(x) => x,
        };

        if vt != &K::get_type(){
            return Err(ExecutionErr::WrongPKType { expected: K::get_type(), got: vt.clone() });
        }

        let tb: Table<K> = Table::new_empty(self.st.schema, self.st.key_name, self.st.table_name.clone());
        if  self.db.insert(self.st.table_name.clone(), tb).is_some(){
            return Err(ExecutionErr::TableAlreadyPresent(self.st.table_name.clone()))
        }

        Ok(())
    }
}




#[cfg(test)]
pub mod test{
    use std::collections::HashMap;

    use super::*;
    use crate::{model::ValueType, parsing::*};
    #[test]
    pub fn test() {
        let query =
            "CREATE a KEY a FIELDS a: String, c: Int, 
            CREATE b KEY a FIELDS a: String, b: Float c: Bool";

        let sts = SQLParser::parse_sql(query).unwrap();
        let mut db: Database<String> = Database::new();

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

        let tables = db.get_tables();

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
}