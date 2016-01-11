#[derive(Debug)]
pub struct Root {
    pub nodes: Vec<Node>
}

#[derive(Debug)]
pub enum Node {
    Comment(String),
    Include(String),
    GlobalDecl(GlobalDecl),
    Statement(Statement)
}

#[derive(Debug)]
pub struct GlobalDecl {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub init_expr: Option<Expr>
}

#[derive(Debug)]
pub enum TypeSpecifier {
    Int,
    Float,
    String
}

#[derive(Debug)]
pub enum Expr {
    String(String),
    FunctionCall(FunctionCall)
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub arguments: ArgumentList
}

pub type ArgumentList = Vec<Expr>;

#[derive(Debug)]
pub enum Statement {
    FunctionCall(FunctionCall),
    If(If)
}

#[derive(Debug)]
pub struct If {
    pub condition: Expr,
    pub body: StatementList,
    pub else_clause: Option<ElseClause>
}

pub type StatementList = Vec<Statement>;

#[derive(Debug)]
pub struct ElseClause {
    pub condition: Expr,
    pub body: StatementList
}
