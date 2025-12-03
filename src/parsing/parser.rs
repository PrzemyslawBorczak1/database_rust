pub use pest::Parser;
pub use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./src/parsing/sql.pest"]

pub struct SQLParser;