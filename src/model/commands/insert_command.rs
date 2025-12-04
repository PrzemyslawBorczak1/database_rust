use crate::errors::DatabaseResult;
use crate::model::{ Database, DatabaseKey, Table};
use crate::parsing::InsertSt;

use super::Command;



#[derive(Debug)]
pub struct Insert<'a, K :DatabaseKey>{
    tb: &'a mut Table<K>,
    st: InsertSt,
}


impl<'a, K : DatabaseKey> Insert<'a, K> {
    pub fn build_exec(db : &mut Database<K>, st: InsertSt) -> DatabaseResult<()>{
        todo!();
    }


    pub fn new(tb: &'a mut Table<K>, st: InsertSt) -> Self{
        Self { tb, st }
    }
}


impl<'a, K : DatabaseKey> Command for Insert<'a ,K > {
    fn execute(self) -> crate::errors::ExecutionResult<()> {
        todo!()
    }
}