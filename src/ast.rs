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
    pub init_expr: Option<Box<Expr>>
}

#[derive(Debug)]
pub enum TypeSpecifier {
    Int,
    Float,
    String
}

#[derive(Debug)]
pub enum Expr {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    StringLiteral(String),
    FunctionCall(FunctionCall),
    VariableRef(VariableRef),
    BinOp(BinOp)
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub arguments: ArgumentList
}

pub type ArgumentList = Vec<Box<Expr>>;

#[derive(Debug)]
pub struct VariableRef {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>
}

#[derive(Debug)]
pub struct BinOp {
    pub op: Op,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>
}

#[derive(Debug)]
pub enum Op {
    Equality,

    Add,
    Mul,
    Div
}

#[derive(Debug)]
pub enum Statement {
    If(If),
    FunctionCall(FunctionCall),
    VariableAssignment(VariableAssignment)
}

#[derive(Debug)]
pub struct If {
    pub condition: Box<Expr>,
    pub body: StatementList,
    pub else_clause: Option<ElseClause>
}

pub type StatementList = Vec<Statement>;

#[derive(Debug)]
pub struct ElseClause {
    pub body: StatementList
}

#[derive(Debug)]
pub struct VariableAssignment {
    pub variable: VariableRef,
    pub expr: Box<Expr>
}
