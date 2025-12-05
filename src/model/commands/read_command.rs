
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
            Err(e) => return Err(ExecutionErr::CouldntReadFile(path.to_string()))
        };

        match SQLParser::parse_sql(&file){
            Ok(_) => Ok(()),
            Err(e) => Err(ExecutionErr::ExecutingError(Box::new(e)))
        }
        
    }
}