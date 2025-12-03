use thiserror::Error;

pub type DatabaseResult<T> = std::result::Result<T, DatabaseError>;

use super::{ParsingError};


#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    ParsingError(#[from]ParsingError),

}