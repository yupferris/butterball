use super::value::*;

#[derive(Debug)]
pub struct Program {
    pub globals: Vec<Variable>,
    pub function_table: Vec<Function>
}

#[derive(Debug)]
pub enum Variable {
    SingleVariable(SingleVariable),
    Array(Array)
}

#[derive(Debug)]
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
    pub name: String
}
