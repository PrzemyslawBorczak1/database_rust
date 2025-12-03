
#[derive(Debug)]
pub enum Statement{
    NoStatement,
    Create(CreateSt),
    Insert(InsertSt)
}


#[derive(Debug)]
pub struct CreateSt{

}

#[derive(Debug)]
pub struct InsertSt{

}