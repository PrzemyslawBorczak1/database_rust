use std::collections::HashMap;
use crate::model::{ValueType};



#[derive(Debug)]
pub enum Statement{
    NoStatement,
    Create(CreateSt),
    Insert(InsertSt)
}


#[derive(Debug, PartialEq)]
pub struct CreateSt{
    table_name: String,
    key_name: String,
    fields: HashMap<String,ValueType>,
}
impl CreateSt{
    pub fn new(table_name: String,  key_name: String, fields: HashMap<String,ValueType>) -> Self{
        Self { 
            table_name,
            key_name,
            fields 
        }
    }
}



#[derive(Debug)]
pub struct InsertSt{

}





#[cfg(test)]
pub mod test{
    use std::collections::HashMap;

    use super::super::*;
    use crate::model::*;

    


    #[test]
    pub fn build_create_statement_test(){
        let query = 
                "CREATE library KEY id FIELDS id: String, title: int, 
                CREATE lib Key i F i:float j: Bool";

        let sts = SQLParser::parse_sql(query).unwrap();
        

        let fields= HashMap::from([
            ("id".to_string(), ValueType::String),
            ("title".to_string(), ValueType::Int),
        ]);
        let should = CreateSt::new("library".to_string(), "id".to_string(), fields);

        if let  Statement::Create(x) = &sts[0]{
            assert_eq!(&should, x,"Incorrect create statement");
        }
        else {
            panic!("Incorrect create statement type");
        }

        
        let fields= HashMap::from([
            ("i".to_string(), ValueType::Float),
            ("j".to_string(), ValueType::Bool),
        ]);
        let should = CreateSt::new("lib".to_string(), "i".to_string(), fields);

        if let  Statement::Create(x) = &sts[1]{
            assert_eq!(&should, x,"Incorrect create statement");
        }
        else {
            panic!("Incorrect create statement type");
        }
    }

}