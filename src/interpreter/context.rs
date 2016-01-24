use super::super::ast;
use super::program_state::*;
use super::impls;

use std::collections::HashMap;

pub struct Context {
    pub function_table: FunctionTable,
    variable_frames: Vec<VariableTable>,

    pub program_state: ProgramState
}

impl Context {
    pub fn new(ast: &ast::Root) -> Context {
        Context {
            function_table: build_function_table(ast),
            variable_frames: vec![HashMap::new()],

            program_state: ProgramState::default()
        }
    }

    pub fn push_variable_frame(&mut self) {
        self.variable_frames.push(HashMap::new());
    }

    pub fn pop_variable_frame(&mut self) {
        self.variable_frames.pop();
    }

    pub fn add_variable(&mut self, name: String, entry: VariableTableEntry) {
        let frame_index = self.variable_frames.len() - 1;
        let mut frame = &mut self.variable_frames[frame_index];
        frame.insert(name, entry);
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

pub type VariableTable = HashMap<String, VariableTableEntry>;

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

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub is_const: bool,
    pub value: Value
}

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

    /*pub fn is_unit(&self) -> bool {
        if let &Value::Unit = self { true } else { false }
    }*/

    pub fn is_integer(&self) -> bool {
        if let &Value::Integer(_) = self { true } else { false }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            &Value::Integer(value) => value,
            _ => panic!("Value was not an integer: {:?}", self)
        }
    }

    /*pub fn is_string(&self) -> bool {
        if let &Value::String(_) = self { true } else { false }
    }*/

    pub fn as_string(&self) -> String {
        match self {
            &Value::String(ref value) => value.clone(),
            _ => panic!("Value was not a string: {:?}", self)
        }
    }
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
