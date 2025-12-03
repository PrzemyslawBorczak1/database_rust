use thiserror::Error;
use crate::parsing::{Rule};


#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Pest parrser error (format of command or parse grammar is incorrect)\n\nerror:\n{0:#?}")]
    PestError(pest::error::Error<Rule>),

    
    #[error("Unrecognized rule {0:#?} in query")]
    UnknownRule(Rule),

    

}