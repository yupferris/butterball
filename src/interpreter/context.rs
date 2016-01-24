use super::super::ast;
use super::program_state::*;
use super::impls;

use std::collections::HashMap;

pub struct Context {
    pub function_table: FunctionTable,
    pub un_op_function_table: UnOpFunctionTable,
    pub bin_op_function_table: BinOpFunctionTable,

    stack_frames: Vec<VariableTable>,

    pub program_state: ProgramState
}

impl Context {
    pub fn new(ast: &ast::Root) -> Context {
        Context {
            function_table: build_function_table(ast),
            un_op_function_table: build_un_op_function_table(),
            bin_op_function_table: build_bin_op_function_table(),

            stack_frames: vec![HashMap::new()],

            program_state: ProgramState::default()
        }
    }

    pub fn push_scope(&mut self) {
        self.stack_frames.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.stack_frames.pop();
    }

    pub fn add_variable(&mut self, name: String, entry: VariableTableEntry) {
        let frame_index = self.stack_frames.len() - 1;
        let mut frame = &mut self.stack_frames[frame_index];
        frame.insert(name, entry);
    }

    pub fn add_or_update_variable(&mut self, name: &String, value: Value) {
        if let Some(variable_table_entry) = self.try_resolve_variable(name) {
            match variable_table_entry {
                &mut VariableTableEntry::Variable(ref mut variable) => {
                    variable.value = value;
                },
                _ => panic!("Unsupported variable table entry for assignments: {:?}", variable_table_entry)
            }
            return;
        }
        self.add_variable(name.clone(), VariableTableEntry::Variable(Variable {
            name: name.clone(),
            is_const: false,
            value: value
        }));
    }

    pub fn update_array_elem_ref(&mut self, name: &String, dimensions: Vec<Value>, value: Value) {
        match self.resolve_variable(name) {
            &mut VariableTableEntry::Array(ref mut array) => {
                let mut index = 0;
                let mut dim_multiplier = 0;
                for i in (0..dimensions.len()).rev() {
                    let current_dimension_size = array.dimensions[i];
                    index += dimensions[i].as_integer() + dim_multiplier * current_dimension_size;
                    dim_multiplier *= current_dimension_size;
                }
                array.values[index as usize] = value;
            },
            _ => panic!("Variable wasn't an array: {}", name)
        }
    }

    pub fn resolve_variable(&mut self, name: &String) -> &mut VariableTableEntry {
        match self.try_resolve_variable(name) {
            Some(x) => x,
            _ => panic!("Could not resolve variable: {}", name)
        }
    }

    pub fn try_resolve_variable(&mut self, name: &String) -> Option<&mut VariableTableEntry> {
        // TODO: Walk frames
        let mut index = self.stack_frames.len();
        loop {
            if index == 0 {
                break;
            }

            index -= 1;

            if self.stack_frames[index].contains_key(name) {
                return self.stack_frames[index].get_mut(name);
            }
        }

        None
    }
}

pub type FunctionTable = HashMap<String, FunctionTableEntry>;

pub enum FunctionTableEntry {
    Decl(ast::FunctionDecl),
    Impl(FunctionImpl)
}

pub type FunctionImpl = Box<Fn(&mut Context, &Vec<Value>) -> Value>;

pub type UnOpFunctionTable = HashMap<(ast::Op, ValueType), FunctionImpl>;
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

fn build_un_op_function_table() -> UnOpFunctionTable {
    impls::build_un_op_impls_table().into_iter().collect::<HashMap<_, _>>()
}

fn build_bin_op_function_table() -> BinOpFunctionTable {
    impls::build_bin_op_impls_table().into_iter().collect::<HashMap<_, _>>()
}
