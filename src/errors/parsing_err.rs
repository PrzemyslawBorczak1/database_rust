use std::fmt;

use pest::error;
use thiserror::Error;
use crate::parsing::{Rule};




pub type ParsingResult<T> = std::result::Result<T, ParsingErr>;

#[derive(Error, Debug, Clone)]
pub enum ParsingErr {
    #[error("Pest parrser error (format of command or parse grammar is incorrect)\n\nerror:\n{0:#?}")]
    PestError(error::Error<Rule>),

    
    #[error("Unrecognized rule {0:#?} in query")]
    UnknownRule(Rule),

    #[error("No table name")]
    NoTableName,

    #[error("Not Enough Arguments")]
    NotEnoughArguments,

    #[error("Not Key name specified")]
    NoKeyName,

    #[error("Unexpected rule. Expected '{expected:#?}' got '{got:#?}'")]
    UnexpectedRule   {
            expected: Rule, 
            got:  Rule,
    },

    #[error("No Type specified for field: '{0}'")]
    NoTypeFor(String),
        
    #[error("No matching type found for rule: {0:?}")]
    UnknownTypeForRule(Rule),


        

}

#[derive(Debug, Clone)]
pub enum StatementErr{
    NotSpecified,
    Create,
    Insert,
}



impl fmt::Display for StatementErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatementErr::NotSpecified => "_",
            StatementErr::Create => "CREATE",
            StatementErr::Insert => "INSERT",
        };
        write!(f, "{}", s)
    }
}