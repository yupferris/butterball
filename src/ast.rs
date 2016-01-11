#[derive(Debug)]
pub enum Expr {
    String(String)
}

pub type ArgumentList = Vec<Expr>;

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: ArgumentList
}

#[derive(Debug)]
pub struct GlobalDecl { // TODO: Initial expr
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>
}

#[derive(Debug)]
pub enum TypeSpecifier {
    Int,
    Float,
    String
}

#[derive(Debug)]
pub enum Statement {
    FunctionCall(FunctionCall)
}

#[derive(Debug)]
pub enum Node {
    Comment(String),
    Include(String),
    GlobalDecl(GlobalDecl),
    Statement(Statement)
}

#[derive(Debug)]
pub struct Root {
    pub nodes: Vec<Node>
}
