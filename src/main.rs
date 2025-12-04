
use database::{model::{AnyDatabase, Create, Database}, parsing::SQLParser};

use pest::Parser;
use database::parsing::*;


fn main() {
    let query = 
                "CREATE library KEY id FIELDS id: String, title: Int, 
                CREATE lib Key i F i:String,
                INSERT a = 2,  b = true, c = true, d = 9.33 INTO t1";
    let x = SQLParser::parse( Rule::sql,query);

    
    let sts = SQLParser::parse_sql(query).unwrap();

    let mut db = AnyDatabase::StringDatabase(Database::new());






    if let Statement::Create(st) = sts[0].clone(){
        let cmd = Create::new(& mut db, st);
        let x = cmd.execute();
        println!("{:#?}", x);
        println!("{:#?}", db);

    }


}
