use crate::errors::{DatabaseResult, ExecutionErr, ExecutionResult, StatementErr};
use crate::model::{ Database, DatabaseKey, Record, Table, Value};
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

    pub fn from (db : &'a mut Database<K>, st: InsertSt) -> ExecutionResult<Self>{
        let table = match db.get_table(&st.table_name){
            None => return  Err(ExecutionErr::NoTable(st.table_name.clone())),
            Some(x) => x
        };

        Ok(Self::new(table, st))

    }

    pub fn new(tb: &'a mut Table<K>, st: InsertSt) -> Self{
        Self { table: tb, st }
    }




    fn handle_pk(&self) -> crate::errors::ExecutionResult<(&String, Value, K)> {
        let pk = self.table.get_pk();

        let pk_v = match self.st.fields.get(pk) {
            None => return Err(ExecutionErr::NoPK(pk.clone())),
            Some(x) => x.clone(),
        };

        let val_key = match K::as_key_value(pk_v.clone()) {
            None => return Err(ExecutionErr::WrongPKType {
                expected: K::get_type(),
                got: pk_v.get_type()
            }),
            Some(x) => x,
        };

        if self.table.contains(&val_key) {
            return Err(ExecutionErr::RepeatedRecord(pk_v.clone()));
        }

        Ok((pk, pk_v, val_key))
    }

    fn fill_and_validate(   &self, rec: &mut Record,   pk: &String   ) -> crate::errors::ExecutionResult<()> {
        let schema = self.table.get_schema();

        for (field, val) in self.st.fields.clone() {
            if &field == pk {
                continue;
            }

            let ex_vt = match schema.get(&field) {
                None => return Err(ExecutionErr::NoDef(field)),
                Some(vt) => vt,
            };

            let got_vt = val.get_type();

            if ex_vt != &got_vt {
                return Err(ExecutionErr::BadType {
                    field,
                    got: got_vt,
                    expected: ex_vt.clone(),
                });
            }

            if rec.add(field.clone(), val).is_some() {
                return Err(ExecutionErr::RepeatedColumn(field));
            }
        }

        Ok(())
    }

    fn check_len(&self) -> crate::errors::ExecutionResult<()> {
        let schema = self.table.get_schema();
        if schema.len() != self.st.fields.len() {
            return Err(ExecutionErr::WrongArgLen {
                table: self.st.table_name.clone(),
                expected: schema.len(),
                got: self.st.fields.len()
            });
        }
        Ok(())
    }
    

}

impl<'a, K : DatabaseKey> Command for Insert<'a ,K > {
    fn execute(self) -> crate::errors::ExecutionResult<()> {
        let mut rec = Record::new();
        self.check_len()?;

        let (pk, pk_v, val_key) = self.handle_pk()?;

        self.fill_and_validate(&mut rec, &pk)?;

        if self.table.add_record(val_key, rec).is_some() {
            return Err(ExecutionErr::RepeatedRecord(pk_v.clone()));
        }

        Ok(())
    }
    
}




#[cfg(test)]
pub mod test{
    use std::collections::HashMap;

    use super::*;
    use crate::{model::{Create, Value}, parsing::*};
    #[test]
    pub fn execute_insert_string() {
        let query =
            "CREATE t KEY a FIELDS a: String, c: Bool,  b : Float
            Insert a =  a, b = 1.0 c = false into t
            Insert a =  b, b = 3.0 c = true into t";

        let sts = SQLParser::parse_sql(query).unwrap();
        let mut db: Database<String> = Database::new();

         if let Statement::Create(st) = sts[0].clone() {
            Create::new(&mut db, st).execute().unwrap()
        } else {
            panic!();
        }


        if let Statement::Insert(st) = sts[1].clone() {
            Insert::from(&mut db, st).unwrap().execute().unwrap()
        } else {
            panic!();
        }

        if let Statement::Insert(st) = sts[2].clone() {
            Insert::from(&mut db, st).unwrap().execute().unwrap()
        } else {
            panic!();
        }


        let table = db.get_table(&"t".to_string()).unwrap();
        let rec = table.get_records();

        let exp = HashMap::from([
            ("b".to_string(), Value::Float(1.0)),
            ("c".to_string(), Value::Bool(false)),
        ]);
        assert_eq!(rec.get(&"a".to_string()).unwrap().fields, exp);


         let exp = HashMap::from([
            ("b".to_string(), Value::Float(3.0)),
            ("c".to_string(), Value::Bool(true)),
        ]);
        assert_eq!(rec.get(&"b".to_string()).unwrap().fields, exp);

    }

    #[test]
pub fn execute_insert_int() {
    let query =
        "CREATE t KEY id FIELDS id: Int, qty: Int, price: Int
        Insert id = 1, qty = 5 price = 100 into t
        Insert id = 2, qty = 3 price = 50 into t";

    let sts = SQLParser::parse_sql(query).unwrap();
    let mut db: Database<i64> = Database::new();

    if let Statement::Create(st) = sts[0].clone() {
        Create::new(&mut db, st).execute().unwrap()
    } else {
        panic!();
    }

    if let Statement::Insert(st) = sts[1].clone() {
        Insert::from(&mut db, st).unwrap().execute().unwrap()
    } else {
        panic!();
    }

    if let Statement::Insert(st) = sts[2].clone() {
        Insert::from(&mut db, st).unwrap().execute().unwrap()
    } else {
        panic!();
    }

    let table = db.get_table(&"t".to_string()).unwrap();
    let rec = table.get_records();

    let exp = HashMap::from([
        ("qty".to_string(), Value::Int(5)),
        ("price".to_string(), Value::Int(100)),
    ]);
    assert_eq!(rec.get(&1).unwrap().fields, exp);

    let exp = HashMap::from([
        ("qty".to_string(), Value::Int(3)),
        ("price".to_string(), Value::Int(50)),
    ]);
    assert_eq!(rec.get(&2).unwrap().fields, exp);
}
}