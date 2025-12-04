
use database::{model::{AnyDatabase, Create, Database}, parsing::SQLParser};

use pest::Parser;
use database::parsing::*;


fn main() {
    let query = 
                "CREATE library KEY id FIELDS id: String, title: Int, 
                CREATE lib Key i F i:String,
                Insert id = a title = 1 I library
                Insert id = b title = 1 I library";
    let x = SQLParser::parse( Rule::sql,query);

    
    let mut db = AnyDatabase::StringDatabase(Database::new());
     SQLParser::run_query(query, &mut db).unwrap();



    println!("{db:#?}");


}
