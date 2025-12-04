use pest::{Parser};
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use std::collections::HashMap;
use crate::errors::{DatabaseErr, DatabaseResult, StatementErr, ParsingErr, ParsingResult};
use crate::model::{AnyDatabase, Value, ValueType};
use super::{Statement, CreateSt, InsertSt};

#[derive(Parser)]
#[grammar = "./src/parsing/sql.pest"]

pub struct SQLParser;


impl SQLParser {
    pub fn run_query(query: &str, db : &mut AnyDatabase) -> DatabaseResult<()>{
        let v = Self::parse_sql(query)?;
        match db {
            AnyDatabase::StringDatabase(db) => {
                for st in v{
                    st.run(db)?;
                }
            }
            AnyDatabase::IntDatabase(db ) => {
                for st in v{
                    st.run(db)?;
                }
            }
        }
       


        Ok(())
    }




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

    fn build_statement(pair: Pair<Rule>) -> DatabaseResult<Statement>{
        
        match pair.as_rule(){

            Rule::create => Self::build_create(&mut pair.into_inner()),
            Rule::insert => Self::build_insert(& mut pair.into_inner()),

            Rule::EOI =>  Ok(Statement::NoStatement),

            r => return  Err( DatabaseErr::ParsingError {
                statement: StatementErr::NotSpecified,
                error: ParsingErr::UnknownRule(r),
            })
        }
    }

    fn check_rule(pair : &Pair<'_, Rule>, rule: Rule) -> ParsingResult<()>{
        let p_rule = pair.as_rule();
        if p_rule != rule{
            return Err(ParsingErr::UnexpectedRule {
                    expected: rule, 
                    got: p_rule 
                });
        };
        Ok(())
    }

    fn check_argument<'a>(pair: Option<Pair<'a, Rule>>) -> ParsingResult<Pair<'a, Rule>>{
        match pair {
            None =>  Err(ParsingErr::NotEnoughArguments),
            Some(p) => Ok(p),
        }
    }

    fn map_value_type_rule(rule: Rule) -> ParsingResult<ValueType> {
        match rule {
            Rule::string | Rule::string_type => Ok(ValueType::String),
            Rule::bool | Rule::bool_type  => Ok(ValueType::Bool),
            Rule::int | Rule::int_type  => Ok(ValueType::Int),
            Rule::float | Rule::float_type  => Ok(ValueType::Float),
            _ => Err(ParsingErr::UnknownTypeForRule(rule)),
        }
    }

    fn check_parsing_from_string<T,E>(val : Result<T, E>, st : &str,  vt: ValueType) -> ParsingResult<T>{
      
        match val {
            Ok(x) => Ok(x),
            Err(_) => Err(ParsingErr::ParsingFromString(st.to_string(), vt))
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

        let schema = match Self::parse_create_schema(p){
            Ok(x) => x,
            Err(e) => return Err(DatabaseErr::ParsingError{
                error: e,
                statement: StatementErr::Create,
           })
        };

        Ok(Statement::Create(CreateSt::new(table_name.to_string(), key_name.to_string(), schema)))
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

    fn parse_create_schema(pairs : &mut Pairs<Rule>) -> ParsingResult<HashMap<String, ValueType>>{
        let mut schema = HashMap::new();
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

            let fn_str= field_name.as_str().to_string();
            if schema.contains_key(&fn_str){
                return Err(ParsingErr::RepeatedColumn(fn_str.clone()));
            }

            schema.insert(field_name.as_str().to_string(), vt);
        };
        Ok(schema)
    }
}

// parsing of insert statement
impl SQLParser{
    fn build_insert(p :&mut Pairs<Rule>) -> DatabaseResult<Statement>{
        let fields = Self::parse_insert_assigment(p);
        let fields= match fields{
            Ok(x) => x,
            Err(e) => return Err(DatabaseErr::ParsingError {
                error: e,
                statement: StatementErr::Insert 
            })
        };
        
        let table_name = match Self::parse_tail(p){
            Ok(x) => x,
            Err(e) =>return Err(DatabaseErr::ParsingError {
                error: e,
                statement: StatementErr::Insert
            })

        };

        Ok(Statement::Insert(InsertSt::new(table_name.to_string(), fields)))
    }

    fn parse_insert_assigment(p : &mut Pairs<Rule>) -> ParsingResult<HashMap<String, Value>>{
        let mut fields = HashMap::new();

        let mut iter = Self::check_argument(p.next())?;
        let mut p_rule = iter.as_rule();


        while p_rule == Rule::assigment{
            let mut def = iter.into_inner();
            Self::parse_assigment(&mut def, &mut fields)?;
            
            iter = Self::check_argument(p.next())?;
            p_rule = iter.as_rule();
        };

        Ok(fields)}

    fn parse_assigment(def :&mut Pairs<'_, Rule>, fields: & mut HashMap<String, Value>) -> ParsingResult<()> {
        let field_name = def.next();
        let typ = def.next();

        let (field_name,typ) = match (field_name, typ){
            (None, None | Some(_)) => return Err(ParsingErr::NotEnoughArguments),

            (Some(t) , None) => return Err(ParsingErr::NoValue(t.to_string())),
            
            (Some(f), Some(t)) => (f, t),
        };
        
        Self::check_rule(&field_name, Rule::field_name)?;
        Self::check_rule(&typ, Rule::value)?;

        let assigment_type = Self::check_argument(typ.into_inner().next())?;

        let vt = Self::map_value_type_rule(assigment_type.as_rule())?;
        let v = Self::parse_string_value(assigment_type, vt)?;

        
        
        let fn_str= field_name.as_str().to_string();
        if fields.contains_key(&fn_str){
            return Err(ParsingErr::RepeatedColumn(fn_str.clone()));
        }
        fields.insert(fn_str, v);

        Ok(())

    }

    fn parse_string_value(first: Pair<'_, Rule>, vt: ValueType) -> ParsingResult<Value>{
        let s = first.as_str();

         match vt {
            ValueType::String => {
                Self::check_rule(&first, Rule::string)?;
                Ok(Value::String(s.to_string()))
            }
            ValueType::Int => {
                Self::check_rule(&first, Rule::int)?;
                let ret = s.parse::<i64>();
                let ret = Self::check_parsing_from_string(ret,s, ValueType::Int)?;
                Ok(Value::Int(ret))
            }
            ValueType::Float => {
               Self::check_rule(&first, Rule::float)?;
                let ret = s.parse::<f64>();
                let ret = Self::check_parsing_from_string(ret,s, ValueType::Int)?;
                Ok(Value::Float(ret))  
            }
            ValueType::Bool => Self::parse_bool_pair(first),
        }
    }

    fn parse_bool_pair(first: Pair<'_, Rule>) -> ParsingResult<Value> {
        Self::check_rule(&first, Rule::bool)?;
        let inner = first.into_inner().next();
        let inner = Self::check_argument(inner)?;

        let i_rule = inner.as_rule();
        match i_rule {
            Rule::true_r => Ok(Value::Bool(true)),
            Rule::false_r => Ok(Value::Bool(false)),
            _ => Err(ParsingErr::UnexpectedRule { expected: Rule::true_r, got:  i_rule}),
        }  
    }

    fn parse_tail<'a, 'b>(p : &'a mut Pairs<'b, Rule>) -> ParsingResult<&'b str>{
        let next = Self::check_argument(p.next())?;
        Self::check_rule(&next, Rule::table_name)?;
        
        Ok(next.as_str())
    }


    }




#[cfg(test)]
pub mod test{
    use super::*;
    #[test]
    pub fn build_statement_type_test(){
        let query = 
                "CREATE library KEY id FIELDS id: String, title: String, 
                CREATE lib Key i F i:String,
                INSERT a = 2,  b = true, c = true, d = 9.33 INTO t1";
        let sts = SQLParser::parse_sql(query).unwrap();
        

        assert!(matches!(sts[0], Statement::Create(_)));
        assert!(matches!(sts[1], Statement::Create(_)));
        assert!(matches!(sts[2], Statement::Insert(_)));
        assert!(matches!(sts[3], Statement::NoStatement));
    }
}

