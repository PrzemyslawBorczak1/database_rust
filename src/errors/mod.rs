pub mod parsing_err;
pub mod database_result;


pub use database_result::{DatabaseErr, DatabaseResult};
pub use parsing_err::{ParsingErr,ParsingResult, StatementErr};