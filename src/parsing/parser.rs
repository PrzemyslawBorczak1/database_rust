
use pest::Parser;
use pest::iterators::{Pair};
use pest_derive::Parser;

use crate::errors::{DatabaseResult, DatabaseError, ParsingError};

use super::{Statement, CreateSt, InsertSt};

#[derive(Parser)]
#[grammar = "./src/parsing/sql.pest"]

pub struct SQLParser;


impl SQLParser {
    pub fn parse_sql(query: &str) -> DatabaseResult<Vec<Statement>> {
        let pairs=  SQLParser::parse(Rule::sql,query);
        let pairs = match pairs{
            Ok(x) => x,
            Err(e) => return Err(DatabaseError::ParsingError(ParsingError::PestError(e))),
        };


        let statements = pairs
                .map(|p|  Self::build_statement(p))
                .collect::<DatabaseResult<Vec<_>>>()?;
        

        Ok(statements)
       
    }

    fn build_statement(p: Pair<Rule>) -> DatabaseResult<Statement>{
        match p.as_rule(){
            Rule::create => Self::build_create(),
            Rule::insert => Self::build_insert(),
            Rule::EOI =>  Ok(Statement::NoStatement),
            r => return  Err(DatabaseError::ParsingError(ParsingError::UnknownRule(r)))
        }


    }


    fn build_create() -> DatabaseResult<Statement>{
        Ok(Statement::Create(CreateSt {  }))
    }

     fn build_insert() -> DatabaseResult<Statement>{
        Ok(Statement::Insert(InsertSt {  }))
    }
}


#[cfg(test)]
pub mod test{
    use super::*;
    #[test]
    pub fn build_statementl_test(){
        let query = "CREATE library KEY id FIELDS id: String, title: String, 
                CREATE lib Key i F i:String,
                INSERT a = 2,  b = x, c = true, d = 9.33 INTO t1";
        let sts = SQLParser::parse_sql(query).unwrap();
        

        assert!(matches!(sts[0], Statement::Create(_)));
        assert!(matches!(sts[1], Statement::Create(_)));
        assert!(matches!(sts[2], Statement::Insert(_)));
        assert!(matches!(sts[3], Statement::NoStatement));
        

    }

}