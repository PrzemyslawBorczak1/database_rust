use crate::errors::{DatabaseResult, ExecutionErr, ExecutionResult, StatementErr};
use crate::model::{Database, DatabaseKey, Record, Value};

use crate::parsing::SelectSt;

#[derive(Debug)]
pub struct Select<'a, K: DatabaseKey> {
    db: &'a mut Database<K>,
    st: SelectSt,
}

impl<'a, K: DatabaseKey> Select<'a, K> {
    pub fn build_exec(db: &'a mut Database<K>, st: SelectSt) -> DatabaseResult<String> {
        let s = Self::new(db, st);

        let res = s.execute();
        match res {
            Err(e) => ExecutionErr::wrap_result(Err(e), StatementErr::Save),
            Ok(s) => Ok(s),
        }
    }

    pub fn new(db: &'a mut Database<K>, st: SelectSt) -> Self {
        Self { db, st }
    }
}

type QueryResult = (Vec<String>, Vec<Vec<String>>);

impl<'a, K: DatabaseKey> Select<'a, K> {
    fn collect_rows(&self) -> ExecutionResult<QueryResult> {
        // gets table mentioned in select rows from TABLE
        let tb = self
            .db
            .get_tables()
            .get(&self.st.table_name)
            .ok_or_else(|| ExecutionErr::NoTable(self.st.table_name.clone()))?;

        // gets columns mentioned in select COLUMN from table
        let columns: Vec<String> = if self.st.all_rows {
            let mut cols: Vec<String> = tb.get_schema().keys().cloned().collect();
            cols.sort();
            cols
        } else {
            self.st.rows.clone()
        };

        // checks if the schema of table is the same as this in query
        let schema = tb.get_schema();
        for col in &columns {
            if !schema.contains_key(col) {
                return Err(ExecutionErr::NoDef(col.clone()));
            }
        }

        // gets pk
        let pk = tb.get_pk().clone();

        let mut records: Vec<(&K, &Record)> = tb.get_records().iter().collect();

        // if order by is choosen
        if let Some(ob) = &self.st.order_by {
            if !tb.get_schema().contains_key(&ob.0) {
                return Err(ExecutionErr::NoDef(ob.0.clone()));
            }
            if ob.0 == pk {
                records.sort_by(|(ka, _), (kb, _)| ka.cmp(kb));
            } else {
                records.sort_by(|(_, a), (_, b)| {
                    match (a.fields.get(&ob.0), b.fields.get(&ob.0)) {
                        (Some(x), Some(y)) => x.compare(y),
                        _ => std::cmp::Ordering::Equal,
                    }
                });
            }
        }

        // if limit is choosen
        if let Some(l) = &self.st.limit {
            records.truncate(l.0 as usize);
        }

        // creates proper structure from record
        let rows: Vec<Vec<String>> = records
            .iter()
            .map(|(key, r)| {
                columns
                    .iter()
                    .map(|c| {
                        if *c == pk {
                            key.to_display()
                        } else {
                            r.fields.get(c).map(Value::to_display).unwrap_or_default()
                        }
                    })
                    .collect()
            })
            .collect();

        Ok((columns, rows))
    }

    fn format_data((headers, rows): QueryResult) -> String {
        let mut widths: Vec<usize> = headers.iter().map(String::len).collect();
        for row in &rows {
            for (i, cell) in row.iter().enumerate() {
                widths[i] = widths[i].max(cell.len());
            }
        }

        let render = |cells: &[String]| -> String {
            cells
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{:width$}", c, width = widths[i]))
                .collect::<Vec<_>>()
                .join(" | ")
        };

        let mut out = String::new();
        out.push_str(&render(&headers));
        out.push('\n');
        out.push_str(
            &"-".repeat(widths.iter().sum::<usize>() + 3 * widths.len().saturating_sub(1)),
        );
        for row in &rows {
            out.push('\n');
            out.push_str(&render(row));
        }
        out
    }

    fn execute(self) -> ExecutionResult<String> {
        let data = self.collect_rows()?;
        Ok(Self::format_data(data))
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        model::AnyDatabase,
        parsing::{LimitSt, OrderBySt, SQLParser},
    };

    use super::*;

    fn seeded_db() -> Database<String> {
        let mut db = AnyDatabase::StringDatabase(Database::new());

        let create = "CREATE scientists KEY name FIELDS name: String, publication_count: Int,";
        SQLParser::run_query(create, &mut db).unwrap();

        // celowo pomieszana kolejnosc publication_count: 3, 7, 5, 1
        let inserts = "
            INSERT name = Newton   publication_count = 3 INTO scientists,
            INSERT name = Curie    publication_count = 7 INTO scientists,
            INSERT name = Einstein publication_count = 5 INTO scientists,
            INSERT name = Darwin   publication_count = 1 INTO scientists,
        ";
        SQLParser::run_query(inserts, &mut db).unwrap();

        match db {
            AnyDatabase::StringDatabase(st) => st,
            _ => unreachable!("wlasnie utworzylismy StringDatabase"),
        }
    }

    /// Buduje SELECT dla podanych kolumn i zwraca dane strukturalnie.
    fn run_select(
        db: &mut Database<String>,
        rows: &[&str],
        limit: Option<u64>,
        order_by: Option<&str>,
    ) -> QueryResult {
        let mut st = SelectSt::new(
            rows.iter().map(|s| s.to_string()).collect(),
            false,
            "scientists".to_string(),
        );
        st.limit = limit.map(LimitSt);
        st.order_by = order_by.map(|f| OrderBySt(f.to_string()));

        Select::new(db, st).collect_rows().unwrap()
    }

    #[test]
    fn select_with_limit_without_order_by_returns_first_n_by_key() {
        let mut db = seeded_db();
        // BTreeMap po kluczu (name): Curie(7), Darwin(1), Einstein(5), Newton(3)
        let (headers, rows) = run_select(&mut db, &["publication_count"], Some(2), None);

        assert_eq!(headers, vec!["publication_count"]);
        assert_eq!(rows, vec![vec!["7"], vec!["1"]]); // Curie, Darwin
    }

    #[test]
    fn select_with_order_by_sorts_by_field() {
        let mut db = seeded_db();
        // ORDER_BY publication_count -> 1, 3, 5, 7
        let (_, rows) = run_select(
            &mut db,
            &["publication_count"],
            None,
            Some("publication_count"),
        );

        assert_eq!(rows, vec![vec!["1"], vec!["3"], vec!["5"], vec!["7"]]);
    }

    #[test]
    fn select_with_order_by_and_limit_sorts_then_truncates() {
        let mut db = seeded_db();
        // ORDER_BY publication_count LIMIT 2 -> 1, 3
        let (_, rows) = run_select(
            &mut db,
            &["publication_count"],
            Some(2),
            Some("publication_count"),
        );

        assert_eq!(rows, vec![vec!["1"], vec!["3"]]);
    }

    #[test]
    fn select_without_modifiers_returns_all_in_key_order() {
        let mut db = seeded_db();
        // brak ORDER_BY / LIMIT -> wszystkie wg klucza: Curie(7), Darwin(1), Einstein(5), Newton(3)
        let (_, rows) = run_select(&mut db, &["publication_count"], None, None);

        assert_eq!(rows, vec![vec!["7"], vec!["1"], vec!["5"], vec!["3"]]);
    }

    #[test]
    fn select_star_returns_all_columns_sorted() {
        let mut db = seeded_db();

        let mut st = SelectSt::new(vec![], true, "scientists".to_string());
        st.order_by = Some(OrderBySt("publication_count".to_string()));
        st.limit = Some(LimitSt(1));

        let (headers, rows) = Select::new(&mut db, st).collect_rows().unwrap();

        // SELECT * -> kolumny posortowane alfabetycznie: name, publication_count
        assert_eq!(headers, vec!["name", "publication_count"]);
        // ORDER_BY publication_count LIMIT 1 -> najmniejszy: Darwin(1)
        assert_eq!(rows, vec![vec!["Darwin", "1"]]);
    }

    #[test]
    fn select_unknown_column_errors() {
        let mut db = seeded_db();

        let st = SelectSt::new(
            vec!["nonexistent".to_string()],
            false,
            "scientists".to_string(),
        );
        let err = Select::new(&mut db, st).collect_rows();

        assert!(matches!(err, Err(ExecutionErr::NoDef(_))));
    }

    #[test]
    fn test() {
        let mut db = seeded_db();

        let mut st = SelectSt::new(vec![], true, "scientists".to_string());
        st.order_by = Some(OrderBySt("publication_count".to_string()));
        st.limit = Some(LimitSt(10));

        let headers = Select::new(&mut db, st).execute().unwrap();
        println!("{}", headers);
    }
}
