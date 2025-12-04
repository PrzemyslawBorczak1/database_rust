use crate::errors::{DatabaseResult, ExecutionErr, ExecutionResult, StatementErr};
use crate::model::{ Database, DatabaseKey, Table, Record};
use crate::parsing::InsertSt;

use super::Command;



#[derive(Debug)]
pub struct Insert<'a, K :DatabaseKey>{
    table: &'a mut Table<K>,
    st: InsertSt,
}


impl<'a, K : DatabaseKey> Insert<'a, K> {
    pub fn build_exec(db : &'a mut Database<K>, st: InsertSt) -> DatabaseResult<()>{
        let i = ExecutionErr::wrap_result(
            Self::from(db, st),
            StatementErr::Insert)?;

        ExecutionErr::wrap_result( i.execute(), StatementErr::Insert)
    }

    fn from (db : &'a mut Database<K>, st: InsertSt) -> ExecutionResult<Self>{
        let table = match db.get_table(&st.table_name){
            None => return  Err(ExecutionErr::NoTable(st.table_name.clone())),
            Some(x) => x
        };

        Ok(Self::new(table, st))

    }

    pub fn new(tb: &'a mut Table<K>, st: InsertSt) -> Self{
        Self { table: tb, st }
    }
}

impl<'a, K : DatabaseKey> Command for Insert<'a ,K > {
    fn execute(self) -> crate::errors::ExecutionResult<()> {
        let mut rec = Record::new();

        let schema = self.table.get_schema();

        let expected = schema.len();
        let got = self.st.fields.len();

        if expected != got{
            return Err(ExecutionErr::WrongArgLen{
                table: self.st.table_name.clone(),
                expected,
                got
            })
        }
        
        let pk  = self.table.get_pk();

        let pk_v = match self.st.fields.get(pk){
            None => return Err(ExecutionErr::NoPK(pk.clone())),
            Some(x) => x.clone(),
        };
        
        let val_key = match K::as_key(pk_v.clone()) {
            None =>  return Err(ExecutionErr::WrongPKType { expected: K::get_type(), got:  pk_v.get_type()}),
            Some(x) => x,
        };

        if self.table.contains(&val_key){
            return Err(ExecutionErr::RepeatedRecord(pk_v.clone()));
        }
        


        
        for (field, val) in self.st.fields{
            if &field == pk{
                continue;
            }


            let ex_vt  = match schema.get(&field){
                None => return Err(ExecutionErr::NoDef(field)),
                Some(vt) => vt,
            };


            let got_vt = val.get_type();

            if ex_vt != &got_vt{
                return Err(ExecutionErr::BadType { field, got: got_vt, expected: ex_vt.clone()});
            }


            if rec.add(field.clone(), val).is_some(){
                return Err(ExecutionErr::RepeatedColumn(field.clone()));
            }
        }


        if self.table.add_record(val_key, rec).is_some(){
            return Err(ExecutionErr::RepeatedRecord(pk_v.clone()));
        }


        Ok(())
    }
    
}