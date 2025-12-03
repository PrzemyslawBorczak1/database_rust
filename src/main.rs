
use database::parsing::SQLParser;


fn main() {
    let query = "CREATE library KEY id FIELDS id: String, title: String, 
        CREATE lib Key i F i:String";
    SQLParser::parse_sql( query);

}
