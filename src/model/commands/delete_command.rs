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

    
    pub fn from (db : &'a mut Database<K>, st: DeleteSt) -> ExecutionResult<Self>{
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

#[cfg(test)]
pub mod test{
    
    use std::collections::HashMap;
    use super::super::*;

    use crate::{model::{Database, Value}, parsing::*};

    #[test]
    pub fn execute_delete_string() {
        let query =
            "CREATE t KEY id FIELDS id: String, qty: Int price: Int
            Insert id = a, qty = 5 price = 100 into t
            Insert id = b, qty = 3 price = 50 into t
            DELETE a FROM t
            DELETE b FROM t";

        let sts = SQLParser::parse_sql(query).unwrap();
        let mut db: Database<String> = Database::new();

        if let Statement::Create(st) = sts[0].clone() {
            Create::new(&mut db, st).execute().unwrap()
        } else {
            panic!();
        }

        // Insert 1
        if let Statement::Insert(st) = sts[1].clone() {
            Insert::from(&mut db, st).unwrap().execute().unwrap()
        } else {
            panic!();
        }

        // Insert 2
        if let Statement::Insert(st) = sts[2].clone() {
            Insert::from(&mut db, st).unwrap().execute().unwrap()
        } else {
            panic!();
        }

        // Delete 1
        if let Statement::Delete(st) = sts[3].clone() {
            Delete::new(db.get_table(&"t".to_string()).unwrap(), st).execute().unwrap()
        } else {
            panic!();
        }

        // Delete 2
        if let Statement::Delete(st) = sts[4].clone() {
            Delete::new(db.get_table(&"t".to_string()).unwrap(), st).execute().unwrap()
        } else {
            panic!();
        }

        // Assert table empty
        let table = db.get_table(&"t".to_string()).unwrap();
        let rec = table.get_records();
        assert!(rec.is_empty());
    }

    #[test]
    pub fn execute_delete_int() {
        let query =
            "CREATE t KEY id FIELDS id: Int, qty: Int price: Int
            Insert id = 1, qty = 5 price = 100 into t
            Insert id = 2, qty = 3 price = 50 into t
            DELETE 1 FROM t";

        let sts = SQLParser::parse_sql(query).unwrap();
        let mut db: Database<i64> = Database::new();

        // Create table
        if let Statement::Create(st) = sts[0].clone() {
            Create::new(&mut db, st).execute().unwrap()
        } else {
            panic!();
        }

        // Insert 1
        if let Statement::Insert(st) = sts[1].clone() {
            Insert::from(&mut db, st).unwrap().execute().unwrap()
        } else {
            panic!();
        }

        // Insert 2
        if let Statement::Insert(st) = sts[2].clone() {
            Insert::from(&mut db, st).unwrap().execute().unwrap()
        } else {
            panic!();
        }

        // Delete key 1
        if let Statement::Delete(st) = sts[3].clone() {
            Delete::new(db.get_table(&"t".to_string()).unwrap(), st).execute().unwrap()
        } else {
            panic!();
        }

        // Assert one record remains (key 2) and it has expected fields
        let table = db.get_table(&"t".to_string()).unwrap();
        let rec = table.get_records();
        assert_eq!(rec.len(), 1);
        let expected = HashMap::from([
            ("qty".to_string(), Value::Int(3)),
            ("price".to_string(), Value::Int(50)),
        ]);
        assert_eq!(rec.get(&2_i64).unwrap().fields, expected);
    }
}