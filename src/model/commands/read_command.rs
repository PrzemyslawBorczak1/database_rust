
use crate::errors::{ExecutionErr, ExecutionResult, DatabaseResult, StatementErr};
use crate::model::{Database, DatabaseKey};
use crate::parsing::{ReadSt, SQLParser};
use super::Command;


use std::fs::read_to_string;

#[derive(Debug)]
pub struct Read<'a, K : DatabaseKey>{
    db: &'a mut Database<K>,
    st: ReadSt,
}


impl<'a, K : DatabaseKey> Read<'a, K> {
    pub fn build_exec(db: &'a mut Database<K>, st: ReadSt) -> DatabaseResult<()>{
        let r = Self::new(db, st);

        ExecutionErr::wrap_result( r.execute(), StatementErr::Read)
    }

    pub fn new(db: &'a mut Database<K>, st: ReadSt) -> Self{
        Self{
            db,st
        }
    }
}

impl<'a, K : DatabaseKey> Command for Read<'a, K> {
    fn execute(self) -> ExecutionResult<()> {
        let path = self.st.path.trim();
        
        let file = match read_to_string(path) {
            Ok(x) => x,
            Err(_) => return Err(ExecutionErr::CouldntReadFile(path.to_string()))
        };

        match SQLParser::run(&file,self.db){
            Ok(_) => Ok(()),
            Err(e) => Err(ExecutionErr::ExecutingError(Box::new(e)))
        }
        
    }
}


#[cfg(test)]
pub mod test{
    use super::*;
    use crate::parsing::Statement;
    use crate::parsing::SQLParser;
    
    use std::collections::HashMap;

    use super::*;
    use crate::{model::{Create, Value}, parsing::*};

    #[test]
    pub fn execute_read_string() {
        let query = "READ_FROM ./src/example_files/test1_str.txt";
        let sts = SQLParser::parse_sql(query).unwrap();

        match &sts[0] {
            Statement::Read(actual) => {
                assert_eq!(actual.path, "./src/example_files/test1_str.txt".to_string());
            }
            _ => panic!(),
        }

        let mut db: Database<String> = Database::new();
        if let Statement::Read(st) = sts[0].clone() {
            let res = Read::new(&mut db, st).execute();
            println!("{res:#?}");
            assert!(res.is_ok());
        }
        else {
                panic!();
        }
    }

    #[test]
    pub fn execute_read_int() {
        let query = "READ_FROM ./src/example_files/test2_int.txt";
        let sts = SQLParser::parse_sql(query).unwrap();

        match &sts[0] {
            Statement::Read(actual) => {
                assert_eq!(actual.path, "./src/example_files/test2_int.txt".to_string());
            }
            _ => panic!(),
        }

        let mut db: Database<i64> = Database::new();
        if let Statement::Read(st) = sts[0].clone() {
            let res = Read::new(&mut db, st).execute();
            println!("{res:#?}");
            assert!(res.is_ok());
        } else {
            panic!();
        }
    }

}




