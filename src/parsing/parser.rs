use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use std::collections::HashMap;
use std::ops::Add;

use super::{CreateSt, DeleteSt, InsertSt, ReadSt, SaveSt, Statement};
use super::{LimitSt, OrderBySt, SelectSt};
use crate::errors::{DatabaseErr, DatabaseResult, ParsingErr, ParsingResult, StatementErr};
use crate::model::{AnyDatabase, Database, DatabaseKey, Value, ValueType};

#[derive(Parser)]
#[grammar = "./src/parsing/sql.pest"]
pub struct SQLParser;

impl SQLParser {
    pub fn run_query(query: &str, db: &mut AnyDatabase) -> DatabaseResult<Option<String>> {
        match db {
            AnyDatabase::StringDatabase(db) => Self::run(query, db),
            AnyDatabase::IntDatabase(db) => Self::run(query, db),
        }
    }

    pub fn run<K: DatabaseKey>(
        query: &str,
        db: &mut Database<K>,
    ) -> DatabaseResult<Option<String>> {
        let mut strings: Vec<String> = Vec::new();
        let v = Self::parse_sql(query)?;
        for st in v {
            db.add_log(st.clone());
            if let Some(s) = st.run(db)? {
                strings.push(s);
            }
        }

        if strings.is_empty() {
            Ok(None)
        } else {
            let mut ret = String::new();
            for s in strings {
                ret = ret.add(&s);
                ret.push('\n');
            }

            Ok(Some(ret))
        }
    }

    pub fn parse_sql(query: &str) -> DatabaseResult<Vec<Statement>> {
        let pairs = SQLParser::parse(Rule::sql, query);
        let pairs = match pairs {
            Ok(x) => x,
            Err(e) => {
                return Err(DatabaseErr::ParsingError {
                    statement: StatementErr::NotSpecified,
                    error: (ParsingErr::PestError(Box::new(e))),
                });
            }
        };

        let statements = pairs
            .map(|p| Self::build_statement(p))
            .collect::<DatabaseResult<Vec<_>>>()?;

        Ok(statements)
    }

    fn build_statement(pair: Pair<Rule>) -> DatabaseResult<Statement> {
        match pair.as_rule() {
            Rule::create => Self::build_create(&mut pair.into_inner()),
            Rule::insert => Self::build_insert(&mut pair.into_inner()),
            Rule::delete => Self::build_delete(&mut pair.into_inner()),
            Rule::read_from => Self::build_read_from(&mut pair.into_inner()),
            Rule::save_as => Self::build_save_as(&mut pair.into_inner()),
            Rule::select => Self::build_select(&mut pair.into_inner()),

            Rule::EOI => Ok(Statement::NoStatement),

            r => Err(DatabaseErr::ParsingError {
                statement: StatementErr::NotSpecified,
                error: ParsingErr::UnknownRule(r),
            }),
        }
    }

    fn check_rule(pair: &Pair<'_, Rule>, rule: Rule) -> ParsingResult<()> {
        let p_rule = pair.as_rule();
        if p_rule != rule {
            return Err(ParsingErr::UnexpectedRule {
                expected: rule,
                got: p_rule,
            });
        };
        Ok(())
    }

    fn check_argument<'a>(pair: Option<Pair<'a, Rule>>) -> ParsingResult<Pair<'a, Rule>> {
        match pair {
            None => Err(ParsingErr::NotEnoughArguments),
            Some(p) => Ok(p),
        }
    }

    fn map_value_type_rule(rule: Rule) -> ParsingResult<ValueType> {
        match rule {
            Rule::string | Rule::string_type => Ok(ValueType::String),
            Rule::bool | Rule::bool_type => Ok(ValueType::Bool),
            Rule::int | Rule::int_type => Ok(ValueType::Int),
            Rule::float | Rule::float_type => Ok(ValueType::Float),
            _ => Err(ParsingErr::UnknownTypeForRule(rule)),
        }
    }

    fn check_parsing_from_string<T, E>(
        val: Result<T, E>,
        st: &str,
        vt: ValueType,
    ) -> ParsingResult<T> {
        match val {
            Ok(x) => Ok(x),
            Err(_) => Err(ParsingErr::ParsingFromString(st.to_string(), vt)),
        }
    }

    fn next_pair<'a>(p: &'a mut Pairs<Rule>) -> DatabaseResult<Pair<'a, Rule>> {
        let ret: Pair<'a, Rule> =
            ParsingErr::wrap_result(Self::check_argument(p.next()), StatementErr::Delete)?;
        Ok(ret)
    }
}

// parsing of create statement
impl SQLParser {
    fn build_create(p: &mut Pairs<Rule>) -> DatabaseResult<Statement> {
        let (table_name, key_name) =
            ParsingErr::wrap_result(Self::parse_create_header(p), StatementErr::Create)?;

        let schema = ParsingErr::wrap_result(Self::parse_create_schema(p), StatementErr::Create)?;

        Ok(Statement::Create(CreateSt::new(
            table_name.to_string(),
            key_name.to_string(),
            schema,
        )))
    }

    fn parse_create_header<'a>(pairs: &mut Pairs<'a, Rule>) -> ParsingResult<(&'a str, &'a str)> {
        let mut it = Self::check_argument(pairs.next())?;

        if it.as_rule() != Rule::table_name {
            return Err(ParsingErr::NoTableName);
        }

        let table_name = it.as_str();
        it = Self::check_argument(pairs.next())?;

        if it.as_rule() != Rule::key_name {
            return Err(ParsingErr::NoKeyName);
        }

        let key_name = it.as_str();

        Ok((table_name, key_name))
    }

    fn parse_create_schema(pairs: &mut Pairs<Rule>) -> ParsingResult<HashMap<String, ValueType>> {
        let mut schema = HashMap::new();
        for pair in pairs {
            Self::check_rule(&pair, Rule::definition)?;
            let mut def = pair.into_inner();
            let field_name = def.next();
            let typ = def.next();
            let (field_name, typ) = match (field_name, typ) {
                (None, None | Some(_)) => return Err(ParsingErr::NotEnoughArguments),

                (Some(t), None) => return Err(ParsingErr::NoTypeFor(t.to_string())),

                (Some(f), Some(t)) => (f, t),
            };
            Self::check_rule(&field_name, Rule::field_name)?;
            Self::check_rule(&typ, Rule::TYPE)?;

            let typ = Self::check_argument(typ.into_inner().next())?;
            let vt = Self::map_value_type_rule(typ.as_rule())?;

            let fn_str = field_name.as_str().to_string();
            if schema.contains_key(&fn_str) {
                return Err(ParsingErr::RepeatedColumn(fn_str.clone()));
            }

            schema.insert(field_name.as_str().to_string(), vt);
        }
        Ok(schema)
    }
}

// parsing of insert statement
impl SQLParser {
    fn build_insert(p: &mut Pairs<Rule>) -> DatabaseResult<Statement> {
        let fields =
            ParsingErr::wrap_result(Self::parse_insert_assigment(p), StatementErr::Insert)?;

        let table_name = ParsingErr::wrap_result(Self::parse_tail(p), StatementErr::Insert)?;

        Ok(Statement::Insert(InsertSt::new(
            table_name.to_string(),
            fields,
        )))
    }

    fn parse_insert_assigment(p: &mut Pairs<Rule>) -> ParsingResult<HashMap<String, Value>> {
        let mut fields = HashMap::new();

        let mut iter = Self::check_argument(p.next())?;
        let mut p_rule = iter.as_rule();

        while p_rule == Rule::assigment {
            let mut def = iter.into_inner();
            Self::parse_assigment(&mut def, &mut fields)?;

            iter = Self::check_argument(p.next())?;
            p_rule = iter.as_rule();
        }

        Ok(fields)
    }

    fn parse_assigment(
        def: &mut Pairs<'_, Rule>,
        fields: &mut HashMap<String, Value>,
    ) -> ParsingResult<()> {
        let field_name = def.next();
        let typ = def.next();

        let (field_name, typ) = match (field_name, typ) {
            (None, None | Some(_)) => return Err(ParsingErr::NotEnoughArguments),

            (Some(t), None) => return Err(ParsingErr::NoValue(t.to_string())),

            (Some(f), Some(t)) => (f, t),
        };

        Self::check_rule(&field_name, Rule::field_name)?;
        Self::check_rule(&typ, Rule::value)?;

        let assigment_type = Self::check_argument(typ.into_inner().next())?;

        let vt = Self::map_value_type_rule(assigment_type.as_rule())?;
        let v = Self::parse_string_value(assigment_type, vt)?;

        let fn_str = field_name.as_str().to_string();
        if fields.contains_key(&fn_str) {
            return Err(ParsingErr::RepeatedColumn(fn_str.clone()));
        }
        fields.insert(fn_str, v);

        Ok(())
    }

    fn parse_string_value(first: Pair<'_, Rule>, vt: ValueType) -> ParsingResult<Value> {
        let s = first.as_str();

        match vt {
            ValueType::String => {
                Self::check_rule(&first, Rule::string)?;
                Ok(Value::String(s.to_string()))
            }
            ValueType::Int => {
                Self::check_rule(&first, Rule::int)?;
                let ret = s.parse::<i64>();
                let ret = Self::check_parsing_from_string(ret, s, ValueType::Int)?;
                Ok(Value::Int(ret))
            }
            ValueType::Float => {
                Self::check_rule(&first, Rule::float)?;
                let ret = s.parse::<f64>();
                let ret = Self::check_parsing_from_string(ret, s, ValueType::Int)?;
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
            _ => Err(ParsingErr::UnexpectedRule {
                expected: Rule::true_r,
                got: i_rule,
            }),
        }
    }

    fn parse_tail<'a>(p: &mut Pairs<'a, Rule>) -> ParsingResult<&'a str> {
        let next = Self::check_argument(p.next())?;
        Self::check_rule(&next, Rule::table_name)?;

        Ok(next.as_str())
    }
}

// parsing of delete
impl SQLParser {
    fn build_delete(p: &mut Pairs<Rule>) -> DatabaseResult<Statement> {
        let key = Self::next_pair(p)?.as_str().to_string();

        let table_name = Self::next_pair(p)?.as_str().to_string();

        Ok(Statement::Delete(DeleteSt::new(table_name, key)))
    }
}

// parsing of read from
impl SQLParser {
    fn build_read_from(p: &mut Pairs<Rule>) -> DatabaseResult<Statement> {
        let file_path = Self::next_pair(p)?.as_str().to_string();
        Ok(Statement::Read(ReadSt::new(file_path)))
    }
}

// save as
impl SQLParser {
    fn build_save_as(p: &mut Pairs<Rule>) -> DatabaseResult<Statement> {
        let file_path = Self::next_pair(p)?.as_str().to_string();
        Ok(Statement::Save(SaveSt::new(file_path)))
    }
}

// select
#[derive(Debug)]
enum Modifier {
    Limit(LimitSt),
    OrderBy(OrderBySt),
}

impl SQLParser {
    fn build_select(p: &mut Pairs<Rule>) -> DatabaseResult<Statement> {
        let mut all_rows = false;
        let rows = ParsingErr::wrap_result(
            Self::parse_row_names(p, &mut all_rows),
            StatementErr::Select,
        )?;

        let table_name = ParsingErr::wrap_result(Self::parse_table_name(p), StatementErr::Select)?;

        let mut node = SelectSt::new(rows, all_rows, table_name);

        while let Some(modifier) =
            ParsingErr::wrap_result(Self::parse_modifier(p), StatementErr::Select)?
        {
            match modifier {
                Modifier::Limit(st) => {
                    if node.limit.is_none() {
                        node.limit = Some(st);
                    } else {
                        return Err(Self::repeated_modifier_err());
                    }
                }
                Modifier::OrderBy(st) => {
                    if node.order_by.is_none() {
                        node.order_by = Some(st);
                    } else {
                        return Err(Self::repeated_modifier_err());
                    }
                }
            }
        }

        DatabaseResult::Ok(Statement::Select(node))
    }

    fn repeated_modifier_err() -> DatabaseErr {
        DatabaseErr::ParsingError {
            error: ParsingErr::RepeatedModifier,
            statement: StatementErr::Select,
        }
    }

    fn parse_row_names(pairs: &mut Pairs<Rule>, all_rows: &mut bool) -> ParsingResult<Vec<String>> {
        let head = Self::check_argument(pairs.next())?;
        if Self::check_rule(&head, Rule::all_rows).is_ok() {
            *all_rows = true;
            return Ok(Vec::new());
        }
        Self::check_rule(&head, Rule::row_names)?;
        let mut ret = Vec::new();
        *all_rows = false;
        for p in head.into_inner() {
            ret.push(String::from(p.as_str()));
        }

        ParsingResult::Ok(ret)
    }

    fn parse_table_name(p: &mut Pairs<Rule>) -> ParsingResult<String> {
        let head = Self::check_argument(p.next())?;
        Self::check_rule(&head, Rule::table_name)?;
        Ok(String::from(head.as_str()))
    }

    fn parse_modifier(p: &mut Pairs<Rule>) -> ParsingResult<Option<Modifier>> {
        if let Some(it) = p.next()
            && Self::check_rule(&it, Rule::modifier).is_ok()
        {
            let modifier = Self::check_argument(it.into_inner().next())?;
            match modifier.as_rule() {
                Rule::limit => {
                    return Ok(Some(Self::parse_limit_modifier(
                        &mut modifier.into_inner(),
                    )?));
                }
                Rule::order_by => {
                    return Ok(Some(Self::parse_order_by_modifier(
                        &mut modifier.into_inner(),
                    )?));
                }
                _ => {
                    return Err(ParsingErr::UnrecognizedModifier(String::from(
                        modifier.as_str(),
                    )));
                }
            }
        }
        ParsingResult::Ok(None)
    }

    fn parse_limit_modifier(m: &mut Pairs<'_, Rule>) -> ParsingResult<Modifier> {
        let vt = ValueType::Int;
        let val: Result<u64, _> = m.as_str().parse();
        let nb = Self::check_parsing_from_string(val, m.as_str(), vt)?;
        Ok(Modifier::Limit(LimitSt(nb)))
    }

    fn parse_order_by_modifier(m: &mut Pairs<'_, Rule>) -> ParsingResult<Modifier> {
        Ok(Modifier::OrderBy(OrderBySt(String::from(m.as_str()))))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn build_statement_type_test() {
        let query = "CREATE library KEY id FIELDS id: String, title: String, 
                CREATE lib Key i F i:String,
                INSERT a = 2,  b = true, c = true, d = 9.33 INTO t1
                DELETE a  FROM t1
                READ_FROM ./some/path/to/somewhere
                DELETE b FROM ty
                SAVE path  ";
        let sts = SQLParser::parse_sql(query).unwrap();

        assert!(matches!(sts[0], Statement::Create(_)));
        assert!(matches!(sts[1], Statement::Create(_)));
        assert!(matches!(sts[2], Statement::Insert(_)));
        assert!(matches!(sts[3], Statement::Delete(_)));
        assert!(matches!(sts[4], Statement::Read(_)));
        assert!(matches!(sts[5], Statement::Delete(_)));
        assert!(matches!(sts[6], Statement::Save(_)));
        assert!(matches!(sts[7], Statement::NoStatement));
    }

    #[test]
    pub fn build_statement_should_fail() {
        let query = "CRTE library KEY id FIELDS id: String, title: String,";
        let sts = SQLParser::parse_sql(query);
        assert!(sts.is_err());

        let query = "CREATE library KEY id FIELDS id STRING id STRING";
        let sts = SQLParser::parse_sql(query);
        assert!(sts.is_err());

        let query = "INSERT a = a a = b INTO a";
        let sts = SQLParser::parse_sql(query);
        assert!(sts.is_err());

        let query = "D a ";
        let sts = SQLParser::parse_sql(query);
        assert!(sts.is_err());

        let query = "R  ";
        let sts = SQLParser::parse_sql(query);
        assert!(sts.is_err());
    }

    #[test]
    pub fn build_select_should_work() {
        let query = "SELECT * FROM tabela Order_BY row Limit 12";

        let st = SQLParser::parse_sql(query);

        println!("{:#?}", st);
    }
}
