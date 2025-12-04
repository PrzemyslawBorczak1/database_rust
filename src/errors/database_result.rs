
use thiserror::Error;

use crate::errors::ExecutionErr;

use super::{ParsingErr};

pub type DatabaseResult<T> = std::result::Result<T, DatabaseErr>;

#[derive(Error, Debug, Clone)]
pub enum DatabaseErr {
    #[error("[{}] {error}", statement.to_string())]
    ParsingError {
        #[source]
        error: ParsingErr,
        statement: StatementErr,
    },

    #[error("[{}] {error}", statement.to_string())]
    ExecutionError {
        #[source]
        error: ExecutionErr,
        statement: StatementErr,
    },
}



#[derive(Debug, Clone)]
pub enum StatementErr{
    NotSpecified,
    Create,
    Insert,
}


impl StatementErr{
    pub fn to_string(&self) -> &str{
        match self {
            StatementErr::NotSpecified => "_",
            StatementErr::Create => "CREATE",
            StatementErr::Insert => "INSERT",
        }
    }
}

