
use database::{model::{AnyDatabase, Database}, parsing::{Rule, SQLParser}};
use pest::Parser;


fn main() {
    let query = 
                "CREATE library KEY id FIELDS id: String, title: Int, 
                CREATE lib Key i F i:String,
                Insert id = a title = 1 I library
                Insert id = b title = 1 I library
                READ_FROM C:\\Users\\przem\\Pulpit\\test6.txt
                D b F library
                ";


    let mut db = AnyDatabase::StringDatabase(Database::new());
    if let Err(x) = SQLParser::run_query(query, &mut db){
        println!("{x}");
    }
    

   // println!("{db:#?}");


}
