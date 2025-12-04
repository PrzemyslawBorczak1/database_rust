
use database::{model::{AnyDatabase, Database}, parsing::SQLParser};


fn main() {
    let query = 
                "CREATE library KEY id FIELDS id: String, title: Int, 
                CREATE lib Key i F i:String,
                Insert id = a title = 1 I library
                Insert id = b title = 1 I library
                D b F library
                ";

    let mut db = AnyDatabase::StringDatabase(Database::new());
     SQLParser::run_query(query, &mut db).unwrap();



    println!("{db:#?}");


}
