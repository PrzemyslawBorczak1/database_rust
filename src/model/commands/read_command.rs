
use crate::errors::{DatabaseResult, ExecutionErr, ExecutionResult, StatementErr};
use crate::model::{Database, DatabaseKey};
use crate::parsing::{ReadSt, SQLParser};

use std::fs::read_to_string;

#[derive(Debug)]
pub struct Read<'a, K: DatabaseKey> {
    db: &'a mut Database<K>,
    st: ReadSt,
}

impl<'a, K: DatabaseKey> Read<'a, K> {
    pub fn build_exec(db: &'a mut Database<K>, st: ReadSt) -> DatabaseResult<Option<String>> {
        let r = Self::new(db, st);

        let res = r.execute();
        match res {
            Err(e) => ExecutionErr::wrap_result(Err(e), StatementErr::Read),
            Ok(s) => Ok(s),
        }
    }

    pub fn new(db: &'a mut Database<K>, st: ReadSt) -> Self {
        Self { db, st }
    }
}

impl<'a, K: DatabaseKey> Read<'a, K> {
    fn execute(self) -> ExecutionResult<Option<String>> {
        let path = self.st.path.trim();

        let file = match read_to_string(path) {
            Ok(x) => x,
            Err(_) => return Err(ExecutionErr::BadFile(path.to_string())),
        };

        match SQLParser::run(&file, self.db) {
            Ok(s) => Ok(s),
            Err(e) => Err(ExecutionErr::ExecutingError(Box::new(e))),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::parsing::SQLParser;
    use crate::parsing::Statement;

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
        } else {
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
