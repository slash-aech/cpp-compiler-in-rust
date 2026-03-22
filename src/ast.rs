#[derive (Debug)]
pub enum Expr{
    IntLiteral(i64),
    Identifier(String),
}
#[derive (Debug)]
pub enum Stmt{
    VarDecl{
        name: String,
        value: Expr,
    }
}
#[derive (Debug)]
pub struct Program{
    
}