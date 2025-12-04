
use thiserror::Error;

use crate::model::{ValueType};


pub type ExecutionResult<T> = std::result::Result<T, ExecutionErr>;

#[derive(Error, Debug, Clone)]
pub enum ExecutionErr {
    #[error("Database already has table '{0}'")]
    TableAlreadyPresent(String),

    
    #[error("No definition for primary key '{0}'")]
    NoPK(String),

    
    #[error("Wrong Primary Key type expected '{expected:?}' got '{got:?}'")]
    WrongPKType{
        expected: ValueType,
        got: ValueType,
    },
}