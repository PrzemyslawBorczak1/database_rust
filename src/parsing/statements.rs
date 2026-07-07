use crate::{
    errors::DatabaseResult,
    model::{
        Create, Database, DatabaseKey, Value, ValueType,
        commands::{Delete, Insert, Read, Save, Select},
    },
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Statement {
    NoStatement,
    Create(CreateSt),
    Insert(InsertSt),
    Delete(DeleteSt),
    Read(ReadSt),
    Save(SaveSt),
    Select(SelectSt),
}

impl Statement {
    pub fn run<K: DatabaseKey>(self, db: &mut Database<K>) -> DatabaseResult<Option<String>> {
        match self {
            Statement::NoStatement => {}
            Statement::Insert(i) => Insert::build_exec(db, i)?,
            Statement::Create(c) => Create::build_exec(db, c)?,
            Statement::Delete(d) => Delete::build_exec(db, d)?,
            Statement::Read(r) => Read::build_exec(db, r)?,
            Statement::Save(s) => Save::build_exec(db, s)?,
            Statement::Select(s) => {
                let s = Select::build_exec(db, s)?;
                return Ok(Some(s));
            }
        }

        Ok(None)
    }

    /// Zamienia statement z powrotem na tekst zapytania SQL, ktory da sie
    /// ponownie sparsowac. Uzywane przez SAVE_AS do zapisu logu jako SQL.
    pub fn to_query(&self) -> String {
        match self {
            Statement::NoStatement => String::new(),
            Statement::Create(c) => c.to_query(),
            Statement::Insert(i) => i.to_query(),
            Statement::Delete(d) => d.to_query(),
            Statement::Read(r) => r.to_query(),
            Statement::Save(s) => s.to_query(),
            Statement::Select(s) => s.to_query(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CreateSt {
    pub table_name: String,
    pub key_name: String,
    pub schema: HashMap<String, ValueType>,
}
impl CreateSt {
    pub fn new(table_name: String, key_name: String, schema: HashMap<String, ValueType>) -> Self {
        Self {
            table_name,
            key_name,
            schema,
        }
    }

    pub fn to_query(&self) -> String {
        // sortujemy pola, bo HashMap nie gwarantuje kolejnosci
        let mut fields: Vec<(&String, &ValueType)> = self.schema.iter().collect();
        fields.sort_by(|a, b| a.0.cmp(b.0));

        let fields = fields
            .iter()
            .map(|(name, vt)| format!("{}: {}", name, vt.to_query_name()))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "CREATE {} KEY {} FIELDS {}",
            self.table_name, self.key_name, fields
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InsertSt {
    pub fields: HashMap<String, Value>,
    pub table_name: String,
}

impl InsertSt {
    pub fn new(table_name: String, fields: HashMap<String, Value>) -> Self {
        Self { table_name, fields }
    }

    pub fn to_query(&self) -> String {
        let mut fields: Vec<(&String, &Value)> = self.fields.iter().collect();
        fields.sort_by(|a, b| a.0.cmp(b.0));

        let assigns = fields
            .iter()
            .map(|(name, val)| format!("{} = {}", name, val.to_query_value()))
            .collect::<Vec<_>>()
            .join(", ");

        format!("INSERT {} INTO {}", assigns, self.table_name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeleteSt {
    pub table_name: String,
    pub key: String,
}

impl DeleteSt {
    pub fn new(table_name: String, key: String) -> Self {
        Self { table_name, key }
    }

    pub fn to_query(&self) -> String {
        format!("DELETE {} FROM {}", self.key, self.table_name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReadSt {
    pub path: String,
}

impl ReadSt {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn to_query(&self) -> String {
        format!("READ_FROM {}", self.path)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SaveSt {
    pub path: String,
}

impl SaveSt {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn to_query(&self) -> String {
        format!("SAVE_AS {}", self.path)
    }
}

#[derive(Debug, Clone)]
pub struct SelectSt {
    pub rows: Vec<String>,
    pub all_rows: bool,
    pub table_name: String,
    pub limit: Option<LimitSt>,
    pub order_by: Option<OrderBySt>,
}
#[derive(Debug, Clone)]
pub struct LimitSt(pub u64);

#[derive(Debug, Clone)]
pub struct OrderBySt(pub String);

impl SelectSt {
    pub fn new(rows: Vec<String>, all_rows: bool, table_name: String) -> Self {
        Self {
            rows,
            all_rows,
            table_name,
            limit: None,
            order_by: None,
        }
    }

    pub fn to_query(&self) -> String {
        let cols = if self.all_rows {
            "*".to_string()
        } else {
            self.rows.join(", ")
        };

        let mut query = format!("SELECT {} FROM {}", cols, self.table_name);

        if let Some(ob) = &self.order_by {
            query.push_str(&format!(" ORDER_BY {}", ob.0));
        }
        if let Some(l) = &self.limit {
            query.push_str(&format!(" LIMIT {}", l.0));
        }

        query
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use super::super::*;
    use crate::model::*;

    #[test]
    pub fn build_create_statement_test() {
        let query = "CREATE library KEY id FIELDS id: String, title: Int, 
                CREATE lib Key i F i:float j: Bool k: Int";

        let sts = SQLParser::parse_sql(query).unwrap();

        match &sts[0] {
            Statement::Create(actual) => {
                let expected_fields = HashMap::from([
                    ("id".to_string(), ValueType::String),
                    ("title".to_string(), ValueType::Int),
                ]);
                let expected =
                    CreateSt::new("library".to_string(), "id".to_string(), expected_fields);
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }

        match &sts[1] {
            Statement::Create(actual) => {
                let expected_fields = HashMap::from([
                    ("i".to_string(), ValueType::Float),
                    ("j".to_string(), ValueType::Bool),
                    ("k".to_string(), ValueType::Int),
                ]);
                let expected = CreateSt::new("lib".to_string(), "i".to_string(), expected_fields);
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }
    }

    #[test]
    pub fn build_insert_statement_test() {
        let query = "INsert a = 1 b= 2.0 c = abc d = false into a, 
                INsert a = was b= 0.0 c = c d = true into a";

        let sts = SQLParser::parse_sql(query).unwrap();

        match &sts[0] {
            Statement::Insert(actual) => {
                let expected_fields = HashMap::from([
                    ("a".to_string(), Value::Int(1)),
                    ("b".to_string(), Value::Float(2.0)),
                    ("c".to_string(), Value::String("abc".to_string())),
                    ("d".to_string(), Value::Bool(false)),
                ]);
                let expected = InsertSt::new("a".to_string(), expected_fields);
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }

        match &sts[1] {
            Statement::Insert(actual) => {
                let expected_fields = HashMap::from([
                    ("a".to_string(), Value::String("was".to_string())),
                    ("b".to_string(), Value::Float(0.0)),
                    ("c".to_string(), Value::String("c".to_string())),
                    ("d".to_string(), Value::Bool(true)),
                ]);
                let expected = InsertSt::new("a".to_string(), expected_fields);
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }
    }

    #[test]
    pub fn build_delete_statement_test() {
        let query = "DELETE a FROM b,
            DELETE b FROM a";

        let sts = SQLParser::parse_sql(query).unwrap();

        match &sts[0] {
            Statement::Delete(actual) => {
                let expected = DeleteSt::new("b".to_string(), "a".to_string());
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }

        match &sts[1] {
            Statement::Delete(actual) => {
                let expected = DeleteSt::new("a".to_string(), "b".to_string());
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }
    }

    #[test]
    pub fn build_read_statement_test() {
        let query = "READ_FROM ./data/input.txt
            READ .\\some\\path\\to\\somewhere";

        let sts = SQLParser::parse_sql(query).unwrap();

        match &sts[0] {
            Statement::Read(actual) => {
                let expected = ReadSt::new("./data/input.txt".to_string());
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }

        match &sts[1] {
            Statement::Read(actual) => {
                let expected = ReadSt::new(".\\some\\path\\to\\somewhere".to_string());
                assert_eq!(&expected, actual);
            }
            _ => panic!(),
        }
    }

    #[test]
    pub fn to_query_round_trips() {
        // parsujemy zapytania, zamieniamy z powrotem na SQL, parsujemy ponownie
        // i sprawdzamy, ze reprezentacja tekstowa jest stabilna (idempotentna).
        let query = "
            CREATE t KEY id FIELDS id: String, n: Int, r: Float, ok: Bool
            INSERT id = \"a\", n = 5, r = 3.0, ok = true INTO t
            DELETE \"a\" FROM t
            SELECT n, r FROM t ORDER_BY n LIMIT 2
            SELECT * FROM t
        ";

        let first: Vec<String> = SQLParser::parse_sql(query)
            .unwrap()
            .iter()
            .map(Statement::to_query)
            .filter(|q| !q.is_empty()) // pomijamy EOI / NoStatement
            .collect();

        // Float 3.0 musi zachowac kropke, inaczej wroci jako Int.
        assert!(
            first.iter().any(|q| q.contains("r = 3.0")),
            "Float zgubil kropke: {first:?}"
        );

        // Ponowne sparsowanie wygenerowanego SQL musi dac te sama reprezentacje.
        let rejoined = first.join("\n");
        let second: Vec<String> = SQLParser::parse_sql(&rejoined)
            .unwrap()
            .iter()
            .map(Statement::to_query)
            .filter(|q| !q.is_empty())
            .collect();

        assert_eq!(
            first, second,
            "to_query nie jest stabilne po ponownym parsowaniu"
        );
    }
}
