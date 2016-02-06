use super::value::*;
use super::impls::*;

#[derive(Debug)]
pub struct Program {
    pub globals: Vec<Variable>,
    pub un_op_impls_table: Vec<FunctionImpl>,
    pub bin_op_impls_table: Vec<FunctionImpl>,
    pub function_table: Vec<FunctionTableEntry>,
    pub main_function: Function
}

#[derive(Debug)]
pub enum Variable {
    SingleVariable(SingleVariable),
    Array(Array)
}

impl Variable {
    pub fn name(&self) -> &String {
        match self {
            &Variable::SingleVariable(ref single_variable) => &single_variable.name,
            &Variable::Array(ref array) => &array.name
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            &Variable::SingleVariable(ref single_variable) => single_variable.value_type,
            &Variable::Array(ref array) => array.value_type
        }
    }
}

#[derive(Debug, Clone)]
pub struct SingleVariable {
    pub name: String,
    pub is_const: bool,
    pub value_type: ValueType
}

#[derive(Debug)]
pub struct Array {
    pub name: String,
    pub value_type: ValueType
}

#[derive(Debug)]
pub enum FunctionTableEntry {
    Function(Function),
    FunctionImpl(FunctionImpl)
}

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: Vec<Statement>
}

#[derive(Debug)]
pub struct FunctionSignature {
    pub name: String,
    pub args: Vec<SingleVariable>
}

#[derive(Debug)]
pub enum Statement {
    ArrayAlloc(ArrayAlloc),
    If(If),
    While(While),
    For(For),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    End
}

#[derive(Debug)]
pub struct ArrayAlloc {
    pub array_ref: ArrayRef,
    pub dimensions: Vec<Box<Expr>>,
    pub value_type: ValueType
}

#[derive(Debug)]
pub enum ArrayRef {
    Global(usize)
}

#[derive(Debug)]
pub struct If {
    pub condition: Box<Expr>,
    pub body: Vec<Statement>,
    pub else_clause: Option<Vec<Statement>>
}

#[derive(Debug)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Vec<Statement>
}

#[derive(Debug)]
pub struct For {
    pub initialization: Assignment,
    pub condition: Box<Expr>,
    pub increment: Assignment,
    pub body: Vec<Statement>
}

#[derive(Debug)]
pub struct Assignment {
    pub l_value: LValue,
    pub expr: Box<Expr>
}

#[derive(Debug)]
pub enum LValue {
    VariableRef(VariableRef),
    ArrayElemRef(ArrayElemRef)
}

#[derive(Debug)]
pub enum VariableRef {
    Global(usize),
    Local(usize)
}

#[derive(Debug)]
pub enum ArrayElemRef {
    Global(GlobalArrayElemRef)
}

impl ArrayElemRef {
    pub fn value_type(&self) -> ValueType {
        match self {
            &ArrayElemRef::Global(ref global_array_elem_ref) => global_array_elem_ref.value_type
        }
    }
}

#[derive(Debug)]
pub struct GlobalArrayElemRef {
    pub global_index: usize,
    pub dimensions: Vec<Box<Expr>>,
    pub value_type: ValueType
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_index: usize,
    pub arguments: Vec<Box<Expr>>,
    pub return_type: ValueType
}

#[derive(Debug)]
pub enum Expr {
    Float(f32),
    Integer(i32),
    String(String),
    FunctionCall(FunctionCall),
    ArrayElemRef(ArrayElemRef),
    VariableRef(VariableRef),
    UnOp(UnOp),
    BinOp(BinOp)
}

// TODO: Merge with function call?
#[derive(Debug)]
pub struct UnOp {
    pub impl_index: usize,
    pub expr: Box<Expr>,
    pub return_type: ValueType
}

// TODO: Merge with function call?
#[derive(Debug)]
pub struct BinOp {
    pub impl_index: usize,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub return_type: ValueType
}
