
use crate::errors::{ExecutionErr, ExecutionResult, DatabaseResult, StatementErr};
use crate::model::{Database, DatabaseKey, Table};
use crate::parsing::DeleteSt;
use super::Command;




#[derive(Debug)]
pub struct Delete<'a, K : DatabaseKey>{
    table: &'a mut Table<K>,
    st: DeleteSt,
}


impl<'a, K : DatabaseKey> Delete<'a, K> {
    pub fn build_exec(db: &'a mut Database<K>, st: DeleteSt) -> DatabaseResult<()>{
          let d = ExecutionErr::wrap_result(
            Self::from(db, st),
            StatementErr::Delete)?;

        ExecutionErr::wrap_result( d.execute(), StatementErr::Delete)
    }

    
    fn from (db : &'a mut Database<K>, st: DeleteSt) -> ExecutionResult<Self>{
        let table = match db.get_table(&st.table_name){
            None => return  Err(ExecutionErr::NoTable(st.table_name.clone())),
            Some(x) => x
        };

        Ok(Self::new(table, st))

    }

    pub fn new(tb: &'a mut Table<K>, st: DeleteSt) -> Self{
        Self { table: tb, st }
    }

    
}


impl<'a, K : DatabaseKey> Command for Delete<'a, K>  {
    fn execute(self) -> ExecutionResult<()> {
        
        let k_st = self.st.key;

        let key = match K::as_key_string(k_st.clone()){
            None => return Err(ExecutionErr::CouldntParse(k_st.clone(), K::get_type())),
            Some(x) => x,
        };


        if self.table.delete_record(&key).is_none(){
            return Err(ExecutionErr::NoKeyStr(k_st, self.st.table_name));
        };
        
        Ok(())
    }
}