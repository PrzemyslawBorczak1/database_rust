use crate::errors::DatabaseResult;
use crate::model::{Database, DatabaseKey};

use crate::parsing::SelectSt;

#[derive(Debug)]
pub struct Select<'a, K: DatabaseKey> {
    db: &'a mut Database<K>,
    select: SelectSt,
}

impl<'a, K: DatabaseKey> Select<'a, K> {
    pub fn build_exec(db: &'a mut Database<K>, st: SelectSt) -> DatabaseResult<()> {
        todo!()
    }
}
