
use database::parsing::SQLParser;

use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use database::parsing::*;


fn main() {
    let query = "CREATE library KEY id FIELDS id: String, title: String, 
        CREATE lib Key i F i:String";
    let x = SQLParser::parse( Rule::sql,query);

    println!("{x:#?}")

}
