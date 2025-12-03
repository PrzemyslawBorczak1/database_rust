
use std::collections::HashMap;

use pest::{ParseResult, Parser};
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::errors::{DatabaseErr, DatabaseResult, StatementErr, ParsingErr, ParsingResult};

use crate::model::{ValueType, table};

use super::{Statement, CreateSt, InsertSt};

#[derive(Parser)]
#[grammar = "./src/parsing/sql.pest"]

pub struct SQLParser;


impl SQLParser {
    pub fn parse_sql(query: &str) -> DatabaseResult<Vec<Statement>> {
        let pairs=  SQLParser::parse(Rule::sql,query);
        let pairs = match pairs{
            Ok(x) => x,
            Err(e) => return Err(DatabaseErr::ParsingError{
                statement: StatementErr::NotSpecified,
                error: ParsingErr::PestError(e)} 
            ),
        };


        let statements = pairs
                .map(|p|  Self::build_statement(p))
                .collect::<DatabaseResult<Vec<_>>>()?;
        

        Ok(statements)
       
    }

    fn build_statement(p: Pair<Rule>) -> DatabaseResult<Statement>{
        
        match p.as_rule(){
            Rule::create => Self::build_create(&mut p.into_inner()),
            Rule::insert => Self::build_insert(p.into_inner()),

            Rule::EOI =>  Ok(Statement::NoStatement),

            r => return  Err( DatabaseErr::ParsingError {
                statement: StatementErr::Create,
                error: ParsingErr::UnknownRule(r),
            })
        }
    }

   
}


// parsing of create statement
impl SQLParser{
     fn build_create(p : &mut Pairs<Rule>) -> DatabaseResult<Statement>{
        let (table_name, key_name) = match Self::parse_create_header( p){
           Ok(x) => x,
           Err(e) => return Err(DatabaseErr::ParsingError{
                error: e,
                statement: StatementErr::Create,
           })
        };

        let fields = match Self::parse_create_fields(p){
            Ok(x) => x,
            Err(e) => return Err(DatabaseErr::ParsingError{
                error: e,
                statement: StatementErr::Create,
           })
        };

        Ok(Statement::Create(CreateSt::new(table_name.to_string(), key_name.to_string(), fields)))
    }
    
    fn check_argument<'a>(p: Option<Pair<'a, Rule>>) -> ParsingResult<Pair<'a, Rule>>{
        match p {
            None =>  Err(ParsingErr::NotEnoughArguments),
            Some(p) => Ok(p),
        }
    }

    fn parse_create_header<'a, 'b>(pairs : &'a mut Pairs<'b ,Rule>) -> ParsingResult<(&'b str, &'b str)>{
      
        let mut it = Self::check_argument(pairs.next())?;

        if it.as_rule() != Rule::table_name {
            return Err(ParsingErr::NoTableName);
        }

        let table_name = it.as_str();
        it =  Self::check_argument(pairs.next())?;


        if it.as_rule() != Rule::key_name {
            return Err(ParsingErr::NoKeyName)
        }

        let key_name = it.as_str();

        Ok((table_name, key_name))
    }

    fn check_rule(p : &Pair<'_, Rule>, r: Rule) -> ParsingResult<()>{
        let p_rule = p.as_rule();
        if p_rule != r{
            return Err(ParsingErr::UnexpectedRule {
                    expected: r, 
                    got: p_rule 
                });
        };
        Ok(())
    }

    fn map_value_type_rule(rule: Rule) -> ParsingResult<ValueType> {
        match rule {
            Rule::string_type => Ok(ValueType::String),
            Rule::bool_type => Ok(ValueType::Bool),
            Rule::int_type => Ok(ValueType::Int),
            Rule::float_type => Ok(ValueType::Float),
            _ => Err(ParsingErr::UnknownTypeForRule(rule)),
        }
    }

    fn parse_create_fields(pairs : &mut Pairs<Rule>) -> ParsingResult<HashMap<String, ValueType>>{
        let mut fields = HashMap::new();
        for pair in pairs {

            Self::check_rule(&pair, Rule::definition)?;

            let mut def = pair.into_inner();
            
            let field_name = def.next();
            let typ = def.next();

            let (field_name,typ) = match (field_name, typ){
                (None, None | Some(_)) => return Err(ParsingErr::NotEnoughArguments),

                (Some(t) , None) => return Err(ParsingErr::NoTypeFor(t.to_string())),
                
                (Some(f), Some(t)) => (f, t),
            };
            
            Self::check_rule(&field_name, Rule::field_name)?;
            Self::check_rule(&typ, Rule::TYPE)?;

            let typ = Self::check_argument(typ.into_inner().next())?;
            let vt = Self::map_value_type_rule(typ.as_rule())?;
            fields.insert(field_name.as_str().to_string(), vt);


        };
        Ok(fields)
    }

}

// parsing of insert statement
impl SQLParser{
      fn build_insert(p : Pairs<Rule>) -> DatabaseResult<Statement>{
        todo!()
    }
}




#[cfg(test)]
pub mod test{
    use super::*;
    #[test]
    pub fn build_statement_type_test(){
        let query = 
                "CREATE library KEY id FIELDS id: String, title: String, 
                CREATE lib Key i F i:String,";
              //  INSERT a = 2,  b = x, c = true, d = 9.33 INTO t1";
        let sts = SQLParser::parse_sql(query).unwrap();
        

        assert!(matches!(sts[0], Statement::Create(_)));
        assert!(matches!(sts[1], Statement::Create(_)));
     //   assert!(matches!(sts[2], Statement::Insert(_)));
        assert!(matches!(sts[2], Statement::NoStatement));
    }


}