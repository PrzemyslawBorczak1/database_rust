
use crate::errors::{ExecutionErr, ExecutionResult, DatabaseResult, StatementErr};
use crate::model::{Database, DatabaseKey};
use crate::parsing::{SaveSt};
use super::Command;

use std::fs::File;
use std::io::{self, Write};


#[derive(Debug)]
pub struct Save<'a, K : DatabaseKey>{
    db: &'a mut Database<K>,
    st: SaveSt,
}


impl<'a, K : DatabaseKey> Save<'a, K> {
    pub fn build_exec(db: &'a mut Database<K>, st: SaveSt) -> DatabaseResult<()>{
        let s = Self::new(db, st);

        ExecutionErr::wrap_result( s.execute(), StatementErr::Save)
    }

    pub fn new(db: &'a mut Database<K>, st: SaveSt) -> Self{
        Self{
            db,st
        }
    }
}


impl<'a, K : DatabaseKey> Command for Save<'a, K> {
    fn execute(self) -> ExecutionResult<()> {
        let file = File::create(self.st.path.clone());
        let mut file = match file{
            Ok(f) => f,
            Err(_) =>return Err(ExecutionErr::BadFile(self.st.path)),
        };
        
        for st in self.db.get_log(){
            match writeln!(file, "{}", st.to_query()) {
                Ok(_) => {},
                Err(_) => return Err(ExecutionErr::BadFile(self.st.path))
            }
        };

        Ok(())
    }
}






#[cfg(test)]
pub mod test{
    use std::collections::HashMap;

    use super::*;
    use crate::model::Create;
    use crate::model::ValueType;
    use crate::parsing::CreateSt;
    use crate::parsing::Statement;
    use crate::parsing::SQLParser;
    

    #[test]
    pub fn execute_save() {
        let mut db: Database<String>= Database::new();

        let schema = HashMap::from([
            ("key".to_string(), ValueType::String),
            ("f".to_string(), ValueType::String)
        ]);
        let st = CreateSt::new("t".to_string(), "key".to_string(),schema);
        let c = Create::new(&mut db, st);
        c.execute().unwrap();



        let st = SaveSt::new("path".to_string());
        let sv = Save::new(&mut db, st);
        println!("{:#?}",  sv.execute());
    }


}