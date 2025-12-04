
use thiserror::Error;

use crate::errors::{DatabaseErr, DatabaseResult, StatementErr};
use crate::model::{ Value, ValueType};


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


    #[error("No table '{0}' in database'")]
    NoTable(String),

    
    #[error("Bad amount of arguments in '{table}', expected {expected} but got {got}")]
    WrongArgLen{
        table: String,
        got: usize,
        expected: usize
    },

      
    #[error("Bad type for '{field}' , expected {expected:?} but got {got:?}")]
    BadType{
        field: String,
        got: ValueType,
        expected: ValueType
    },

    #[error("No field definition in schema for {0}")]
    NoDef(String),

    
    #[error("Column '{0}' appeared more than once")]
    RepeatedColumn(String),

    
    #[error("Value '{0:?}' appeared more than once")]
    RepeatedRecord(Value),


    
}


impl ExecutionErr{
    pub fn wrap_result<T>(er : ExecutionResult<T>, st : StatementErr) -> DatabaseResult<T>{
        match er {
            Ok(x) => Ok(x),
            Err(e) => Err(DatabaseErr::ExecutionError { error: e, statement: st }) 
        }
    }
}