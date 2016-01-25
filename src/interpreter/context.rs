use super::super::ast;
use super::program_state::*;
use super::impls;

use std::collections::HashMap;

pub struct Context {
    pub function_table: FunctionTable,
    pub un_op_function_table: UnOpFunctionTable,
    pub bin_op_function_table: BinOpFunctionTable,

    pub data_table: Vec<Value>,
    pub data_labels: HashMap<String, usize>,

    stack_frames: Vec<VariableTable>,

    pub program_state: ProgramState
}

impl Context {
    pub fn new(ast: &ast::Root) -> Context {
        let (data_table, data_labels) = build_data_tables(ast);

        Context {
            function_table: build_function_table(ast),
            un_op_function_table: build_un_op_function_table(),
            bin_op_function_table: build_bin_op_function_table(),

            data_table: data_table,
            data_labels: data_labels,

            stack_frames: vec![HashMap::new()],

            program_state: ProgramState::new()
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

    pub fn add_or_update_variable(&mut self, name: &String, value: Value, value_type: ValueType) {
        if let Some(variable_table_entry) = self.try_resolve_variable(name) {
            match variable_table_entry {
                &mut VariableTableEntry::Variable(ref mut variable) => {
                    variable.value = value.cast_to(&variable.value_type);
                },
                _ => panic!("Unsupported variable table entry for assignments: {:?}", variable_table_entry)
            }
            return;
        }
        self.add_variable(name.clone(), VariableTableEntry::Variable(Variable {
            name: name.clone(),
            is_const: false,
            value: value.cast_to(&value_type),
            value_type: value_type
        }));
    }

    pub fn update_array_elem_ref(&mut self, name: &String, dimensions: Vec<Value>, value: Value) {
        match self.resolve_variable(name) {
            &mut VariableTableEntry::Array(ref mut array) => {
                *array.index(&dimensions) = value.cast_to(&array.value_type);
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
    pub value: Value,
    pub value_type: ValueType
}

#[derive(Debug)]
pub struct Array {
    pub name: String,
    pub dimensions: Vec<i32>,
    pub values: Vec<Value>,
    pub value_type: ValueType
}

impl Array {
    pub fn index(&mut self, dimensions: &Vec<Value>) -> &mut Value {
        let mut index = 0;
        let mut dim_multiplier = 1;
        for i in (0..dimensions.len()).rev() {
            let current_dimension_size = self.dimensions[i];
            index += dimensions[i].cast_to_integer().as_integer() * dim_multiplier;
            dim_multiplier *= current_dimension_size;
        }
        &mut self.values[index as usize]
    }
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
    pub fn default(value_type: &ValueType) -> Value {
        match value_type {
            &ValueType::Unit => Value::Unit,
            &ValueType::Integer => Value::Integer(0),
            &ValueType::Float => Value::Float(0.0),
            &ValueType::Bool => Value::Bool(false),
            &ValueType::String => Value::String(String::new())
        }
    }

    pub fn cast_to_unit(&self) -> Value {
        match self {
            &Value::Unit => Value::Unit,
            _ => panic!("Value cannot be cast to a unit: {:?}", self)
        }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            &Value::Integer(value) => value,
            _ => panic!("Value was not an integer: {:?}", self)
        }
    }

    pub fn cast_to_integer(&self) -> Value {
        Value::Integer(match self {
            &Value::Integer(value) => value,
            &Value::Float(value) => value as i32,
            &Value::Bool(value) => if value { 1 } else { 0 },
            _ => panic!("Value cannot be cast to an integer: {:?}", self)
        })
    }

    pub fn as_float(&self) -> f32 {
        match self {
            &Value::Float(value) => value,
            _ => panic!("Value was not an float: {:?}", self)
        }
    }

    pub fn cast_to_float(&self) -> Value {
        Value::Float(match self {
            &Value::Integer(value) => value as f32,
            &Value::Float(value) => value,
            _ => panic!("Value cannot be cast to a float: {:?}", self)
        })
    }

    pub fn as_bool(&self) -> bool {
        match self {
            &Value::Bool(value) => value,
            _ => panic!("Value was not a bool: {:?}", self)
        }
    }

    pub fn cast_to_bool(&self) -> Value {
        Value::Bool(match self {
            &Value::Bool(value) => value,
            _ => panic!("Value cannot be cast to an integer: {:?}", self)
        })
    }

    pub fn as_string(&self) -> String {
        match self {
            &Value::String(ref value) => value.clone(),
            _ => panic!("Value was not a string: {:?}", self)
        }
    }

    pub fn cast_to_string(&self) -> Value {
        Value::String(match self {
            &Value::String(ref value) => value.clone(),
            _ => panic!("Value cannot be cast to a string: {:?}", self)
        })
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

    pub fn cast_to(&self, value_type: &ValueType) -> Value {
        match value_type {
            &ValueType::Unit => self.cast_to_unit(),
            &ValueType::Integer => self.cast_to_integer(),
            &ValueType::Float => self.cast_to_float(),
            &ValueType::Bool => self.cast_to_bool(),
            &ValueType::String => self.cast_to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Unit,
    Integer,
    Float,
    Bool,
    String
}

impl<'a> From<&'a Option<ast::TypeSpecifier>> for ValueType {
    fn from(type_specifier: &Option<ast::TypeSpecifier>) -> Self {
        match type_specifier {
            &Some(ast::TypeSpecifier::Int) | &None => ValueType::Integer,
            &Some(ast::TypeSpecifier::Float) => ValueType::Float,
            &Some(ast::TypeSpecifier::String) => ValueType::String,
            _ => panic!("Unrecognized type specifier: {:?}", type_specifier)
        }
    }
}

fn build_data_tables(root: &ast::Root) -> (Vec<Value>, HashMap<String, usize>) {
    let mut data_table = Vec::new();
    let mut data_labels = HashMap::new();

    for node in root.nodes.iter() {
        match node {
            &ast::Node::Label(ref name) => {
                data_labels.insert(name.clone(), data_table.len());
            },
            &ast::Node::Data(ref data) => {
                for value in data.values.iter() {
                    data_table.push(match **value {
                        ast::Expr::FloatLiteral(x) => Value::Float(x),
                        ast::Expr::IntegerLiteral(x) => Value::Integer(x),
                        ast::Expr::BoolLiteral(x) => Value::Bool(x),
                        ast::Expr::StringLiteral(ref x) => Value::String(x.clone()),

                        _ => panic!("Invalid data value: {:?}", value)
                    });
                }
            },
            _ => ()
        }
    }

    (data_table, data_labels)
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
