#[derive(Debug)]
pub struct Root {
    pub nodes: Vec<Node>
}

// TODO: Better name
#[derive(Debug)]
pub enum Node {
    Include(String),
    TypeDecl(TypeDecl),
    GlobalVariableDecl(VariableDecl),
    ConstDecl(ConstDecl),
    FunctionDecl(FunctionDecl),
    Statement(Statement),
    Label(String),
    Data(Data),
    End
}

#[derive(Debug)]
pub struct TypeDecl {
    pub name: String,
    pub fields: Vec<Field>
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub array_size: Option<i32>
}

#[derive(Debug, Clone)]
pub enum TypeSpecifier {
    Int,
    Float,
    String,
    Custom(String)
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub init_expr: Option<Box<Expr>>
}

#[derive(Debug)]
pub struct ConstDecl {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub init_expr: Box<Expr>
}

#[derive(Debug, Clone)]
pub struct ArrayDecl {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub dimensions: Vec<Box<Expr>>
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub args: Vec<VariableDecl>,
    pub body: StatementList
}

#[derive(Debug, Clone)]
pub enum Expr {
    FloatLiteral(f32),
    IntegerLiteral(i32),
    BoolLiteral(bool),
    StringLiteral(String),
    FunctionCallOrArrayElemRef(FunctionCallOrArrayElemRef),
    VariableRef(VariableRef),
    UnOp(UnOp),
    BinOp(BinOp)
}

#[derive(Debug, Clone)]
pub struct FunctionCallOrArrayElemRef {
    pub function_name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub arguments: ArgumentList
}

pub type ArgumentList = Vec<Box<Expr>>;

#[derive(Debug, Clone)]
pub struct VariableRef {
    pub name: String,
    pub type_specifier: Option<TypeSpecifier>
}

#[derive(Debug, Clone)]
pub struct UnOp {
    pub op: Op,
    pub expr: Box<Expr>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
    Not,

    Eq,
    Neq,

    And,
    Or,
    Xor,

    LtEq,
    GtEq,
    Lt,
    Gt,

    Add,
    Sub,

    Mul,
    Div,

    Shl,
    Shr,
    Sar,

    Neg
}

#[derive(Debug, Clone)]
pub struct BinOp {
    pub op: Op,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>
}

#[derive(Debug, Clone)]
pub enum Statement {
    ArrayDecl(ArrayDecl),
    If(If),
    While(While),
    Repeat(Repeat),
    For(For),
    Select(Select),
    Restore(String),
    Read(LValue),
    Assignment(Assignment),
    FunctionCall(FunctionCallOrArrayElemRef)
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expr>,
    pub body: StatementList,
    pub else_clause: Option<ElseClause>
}

pub type StatementList = Vec<Statement>;

#[derive(Debug, Clone)]
pub struct ElseClause {
    pub body: StatementList
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: StatementList
}

#[derive(Debug, Clone)]
pub struct Repeat {
    pub body: StatementList
}

#[derive(Debug, Clone)]
pub struct For {
    pub initialization: Assignment,
    pub to: Box<Expr>,
    pub step: Option<Box<Expr>>,
    pub body: StatementList
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub l_value: LValue,
    pub expr: Box<Expr>
}

#[derive(Debug, Clone)]
pub enum LValue {
    VariableRef(VariableRef),
    ArrayElemRef(ArrayElemRef)
}

// TODO: Remove
impl LValue {
    pub fn as_variable_ref(&self) -> VariableRef {
        match self {
            &LValue::VariableRef(ref variable_ref) => variable_ref.clone(),
            _ => panic!("LValue wasn't a variable ref")
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrayElemRef {
    pub array_name: String,
    pub type_specifier: Option<TypeSpecifier>,
    pub dimensions: Vec<Box<Expr>>
}

#[derive(Debug, Clone)]
pub struct Select {
    pub expr: Box<Expr>,
    pub arms: Vec<CaseArm>
}

#[derive(Debug, Clone)]
pub struct CaseArm {
    pub value: Box<Expr>,
    pub body: StatementList
}

#[derive(Debug)]
pub struct Data {
    pub values: Vec<Box<Expr>>
}
