use thiserror::Error;
use crate::errors::StatementErr;

use super::{ParsingErr};

pub type DatabaseResult<T> = std::result::Result<T, DatabaseErr>;

#[derive(Error, Debug, Clone)]
pub enum DatabaseErr {
    #[error("[{statement}] {error}")]
    ParsingError {
        #[source]
        error: ParsingErr,
        statement: StatementErr,
    },
}