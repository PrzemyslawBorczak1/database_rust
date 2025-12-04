use crate::{errors::DatabaseResult, model::{ AnyDatabase, Database, DatabaseKey, Table, table::AnyTableRef}, parsing::InsertSt};





#[derive(Debug)]
pub struct Insert<'a>{
    tb: &'a mut AnyTableRef,
    st: InsertSt,
}


impl<'a> Insert<'a> {
    pub fn build_exec(db : &mut AnyDatabase, st: InsertSt) -> DatabaseResult<()>{
        todo!()
    }


    pub fn new(tb: &'a mut AnyTableRef, st: InsertSt) -> Self{
        Self { tb, st }
    }


    pub fn execute(self) -> DatabaseResult<()>{
       Ok(())
    }
}