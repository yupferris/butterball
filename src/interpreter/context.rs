use super::super::ast;
use super::program_state::*;
use super::impls;

use std::collections::HashMap;

pub struct Context {
    pub function_table: FunctionTable,
    pub bin_op_function_table: BinOpFunctionTable,

    variable_frames: Vec<VariableTable>,

    pub program_state: ProgramState
}

impl Context {
    pub fn new(ast: &ast::Root) -> Context {
        Context {
            function_table: build_function_table(ast),
            bin_op_function_table: build_bin_op_function_table(),
            variable_frames: vec![HashMap::new()],

            program_state: ProgramState::default()
        }
    }

    pub fn push_scope(&mut self) {
        self.variable_frames.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.variable_frames.pop();
    }

    pub fn add_variable(&mut self, name: String, entry: VariableTableEntry) {
        let frame_index = self.variable_frames.len() - 1;
        let mut frame = &mut self.variable_frames[frame_index];
        frame.insert(name, entry);
    }

    pub fn add_or_update_variable(&mut self, name: &String, value: Value) {
        // TODO: Walk frames
        let frame_index = self.variable_frames.len() - 1;
        let mut frame = &mut self.variable_frames[frame_index];
        if let Some(variable_table_entry) = frame.get_mut(name) {
            match variable_table_entry {
                &mut VariableTableEntry::Variable(ref mut variable) => {
                    variable.value = value;
                },
                _ => panic!("Unsupported variable table entry for assignments: {:?}", variable_table_entry)
            }
            return;
        }
        frame.insert(name.clone(), VariableTableEntry::Variable(Variable {
            name: name.clone(),
            is_const: false,
            value: value
        }));
    }

    pub fn resolve_variable(&self, name: &String) -> &VariableTableEntry {
        // TODO: Walk frames
        let frame = &self.variable_frames[self.variable_frames.len() - 1];
        frame.get(name).unwrap()
    }
}

pub type FunctionTable = HashMap<String, FunctionTableEntry>;

pub enum FunctionTableEntry {
    Decl(ast::FunctionDecl),
    Impl(FunctionImpl)
}

pub type FunctionImpl = Box<Fn(&mut Context, &Vec<Value>) -> Value>;

pub type BinOpFunctionTable = HashMap<(ast::Op, ValueType, ValueType), FunctionImpl>;

pub type VariableTable = HashMap<String, VariableTableEntry>;

#[derive(Debug)]
pub enum VariableTableEntry {
    Variable(Variable),
    Array(Array)
}

impl VariableTableEntry {
    pub fn as_variable(&self) -> Variable {
        match self {
            &VariableTableEntry::Variable(ref variable) => variable.clone(),
            _ => panic!("Variable table entry was not a variable")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub is_const: bool,
    pub value: Value
}

#[derive(Debug)]
pub struct Array {
    pub name: String,
    pub dimensions: Vec<i32>,
    pub values: Vec<Value>
}

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Integer(i32),
    Float(f32),
    Bool(bool),
    String(String)
}

impl Value {
    pub fn default(type_specifier: &Option<ast::TypeSpecifier>) -> Value {
        match type_specifier {
            &Some(ast::TypeSpecifier::Int) | &None => Value::Integer(0),
            &Some(ast::TypeSpecifier::Float) => Value::Float(0.0),
            &Some(ast::TypeSpecifier::String) => Value::String(String::new()),
            _ => panic!("Unrecognized type specifier: {:?}", type_specifier)
        }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            &Value::Integer(value) => value,
            _ => panic!("Value was not an integer: {:?}", self)
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            &Value::Float(value) => value,
            _ => panic!("Value was not an float: {:?}", self)
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            &Value::Bool(value) => value,
            _ => panic!("Value was not a bool: {:?}", self)
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            &Value::String(ref value) => value.clone(),
            _ => panic!("Value was not a string: {:?}", self)
        }
    }

    pub fn to_expr(&self) -> Box<ast::Expr> {
        Box::new(match self {
            &Value::Integer(value) => ast::Expr::IntegerLiteral(value),
            &Value::Float(value) => ast::Expr::FloatLiteral(value),
            &Value::Bool(value) => ast::Expr::BoolLiteral(value),
            &Value::String(ref value) => ast::Expr::StringLiteral(value.clone()),
            _ => panic!("Value cannot be represented as an expression: {:?}", self)
        })
    }

    pub fn get_type(&self) -> ValueType {
        match self {
            &Value::Unit => ValueType::Unit,
            &Value::Integer(_) => ValueType::Integer,
            &Value::Float(_) => ValueType::Float,
            &Value::Bool(_) => ValueType::Bool,
            &Value::String(_) => ValueType::String
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ValueType {
    Unit,
    Integer,
    Float,
    Bool,
    String
}

fn build_function_table(root: &ast::Root) -> FunctionTable {
    let mut ret = HashMap::new();

    for node in root.nodes.iter() {
        match node {
            &ast::Node::FunctionDecl(ref function_decl) => {
                ret.insert(function_decl.name.clone(), FunctionTableEntry::Decl(function_decl.clone()));
            },
            _ => ()
        }
    }

    for (name, f) in impls::build_impls_table() {
        ret.insert(String::from(name), FunctionTableEntry::Impl(f));
    }

    ret
}

fn build_bin_op_function_table() -> BinOpFunctionTable {
    impls::build_bin_op_impls_table().into_iter().collect::<HashMap<_, _>>()
}
