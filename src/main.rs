
use database::parsing::SQLParser;

use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use database::parsing::*;


fn main() {
    let query = 
                "CREATE library KEY id FIELDS id: String, title: Int, 
                CREATE lib Key i F i:String,
                INSERT a = 2,  b = true, c = true, d = 9.33 INTO t1";
    let x = SQLParser::parse( Rule::sql,query);

    let x = SQLParser::parse_sql(query);
    println!("{x:#?}")

}
