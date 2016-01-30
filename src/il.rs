use super::value::*;

#[derive(Debug)]
pub struct Program {
    pub globals: Vec<Variable>,
    pub function_table: Vec<Function>,
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
    Todo
}
