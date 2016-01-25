use super::super::ast;
use super::value::*;
use super::context::*;
use super::impls::*;

use std::collections::HashMap;

pub fn interpret(root: &ast::Root) {
    let mut state = State::new(root);
    for node in root.nodes.iter() {
        if interpret_node(&mut state, node) { break; }
    }
}

struct State {
    function_table: FunctionTable,
    un_op_function_table: UnOpFunctionTable,
    bin_op_function_table: BinOpFunctionTable,

    data_table: Vec<Value>,
    data_labels: HashMap<String, usize>,
    data_pointer: usize,

    stack_frames: Vec<VariableTable>,

    context: Context,
}

impl State {
    fn new(root: &ast::Root) -> State {
        let (data_table, data_labels) = build_data_tables(root);

        State {
            function_table: build_function_table(root),
            un_op_function_table: build_un_op_function_table(),
            bin_op_function_table: build_bin_op_function_table(),

            data_table: data_table,
            data_labels: data_labels,
            data_pointer: 0,

            stack_frames: vec![HashMap::new()],

            context: Context::new()
        }
    }

    fn push_scope(&mut self) {
        self.stack_frames.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.stack_frames.pop();
    }

    fn add_variable(&mut self, name: String, entry: VariableTableEntry) {
        let frame_index = self.stack_frames.len() - 1;
        let mut frame = &mut self.stack_frames[frame_index];
        frame.insert(name, entry);
    }

    fn add_or_update_variable(&mut self, name: &String, value: Value, value_type: ValueType) {
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

    fn update_array_elem_ref(&mut self, name: &String, dimensions: Vec<Value>, value: Value) {
        match self.resolve_variable(name) {
            &mut VariableTableEntry::Array(ref mut array) => {
                *array.index(&dimensions) = value.cast_to(&array.value_type);
            },
            _ => panic!("Variable wasn't an array: {}", name)
        }
    }

    fn resolve_variable(&mut self, name: &String) -> &mut VariableTableEntry {
        match self.try_resolve_variable(name) {
            Some(x) => x,
            _ => panic!("Could not resolve variable: {}", name)
        }
    }

    fn try_resolve_variable(&mut self, name: &String) -> Option<&mut VariableTableEntry> {
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

type FunctionTable = HashMap<String, FunctionTableEntry>;

enum FunctionTableEntry {
    Decl(ast::FunctionDecl),
    Impl(FunctionImpl)
}

type UnOpFunctionTable = HashMap<(ast::Op, ValueType), FunctionImpl>;
type BinOpFunctionTable = HashMap<(ast::Op, ValueType, ValueType), FunctionImpl>;

type VariableTable = HashMap<String, VariableTableEntry>;

#[derive(Debug)]
enum VariableTableEntry {
    Variable(Variable),
    Array(Array)
}

impl VariableTableEntry {
    fn as_variable(&self) -> Variable {
        match self {
            &VariableTableEntry::Variable(ref variable) => variable.clone(),
            _ => panic!("Variable table entry was not a variable")
        }
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    is_const: bool,
    value: Value,
    value_type: ValueType
}

#[derive(Debug)]
struct Array {
    name: String,
    dimensions: Vec<i32>,
    values: Vec<Value>,
    value_type: ValueType
}

impl Array {
    fn index(&mut self, dimensions: &Vec<Value>) -> &mut Value {
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

    for (name, f) in build_impls_table() {
        ret.insert(String::from(name), FunctionTableEntry::Impl(f));
    }

    ret
}

fn build_un_op_function_table() -> UnOpFunctionTable {
    build_un_op_impls_table().into_iter().collect::<HashMap<_, _>>()
}

fn build_bin_op_function_table() -> BinOpFunctionTable {
    build_bin_op_impls_table().into_iter().collect::<HashMap<_, _>>()
}

// Returns true if we should end, false otherwise
// TODO: Better way to do that?
fn interpret_node(state: &mut State, node: &ast::Node) -> bool {
    match node {
        &ast::Node::GlobalVariableDecl(ref variable_decl) => interpret_variable_decl(state, variable_decl),
        &ast::Node::ConstDecl(ref const_decl) => interpret_const_decl(state, const_decl),
        &ast::Node::Statement(ref statement) => interpret_statement(state, statement),
        &ast::Node::End => { return true; },
        _ => panic!("Unrecognized node: {:?}", node)
    }
    false
}

fn interpret_variable_decl(state: &mut State, variable_decl: &ast::VariableDecl) {
    let value_type = (&variable_decl.type_specifier).into();
    let value = match &variable_decl.init_expr {
        &Some(ref expr) => eval_expr(state, expr).cast_to(&value_type),
        _ => Value::default(&value_type)
    };
    state.add_variable(variable_decl.name.clone(), VariableTableEntry::Variable(Variable {
        name: variable_decl.name.clone(),
        is_const: false,
        value: value,
        value_type: value_type
    }));
}

// TODO: Can probably reuse more code between this and interpret_variable_decl
fn interpret_const_decl(state: &mut State, const_decl: &ast::ConstDecl) {
    let value_type = (&const_decl.type_specifier).into();
    let value = eval_expr(state, &const_decl.init_expr).cast_to(&value_type);
    state.add_variable(const_decl.name.clone(), VariableTableEntry::Variable(Variable {
        name: const_decl.name.clone(),
        is_const: true,
        value: value,
        value_type: value_type
    }));
}

fn eval_expr(state: &mut State, expr: &Box<ast::Expr>) -> Value {
    match **expr {
        ast::Expr::FloatLiteral(value) => Value::Float(value),
        ast::Expr::IntegerLiteral(value) => Value::Integer(value),
        ast::Expr::BoolLiteral(value) => Value::Bool(value),
        ast::Expr::StringLiteral(ref value) => Value::String(value.clone()),
        ast::Expr::FunctionCall(ref function_call) => eval_function_call(state, function_call),
        ast::Expr::VariableRef(ref variable_ref) => eval_variable_ref(state, variable_ref),
        ast::Expr::UnOp(ref un_op) => eval_un_op(state, un_op),
        ast::Expr::BinOp(ref bin_op) => eval_bin_op(state, bin_op)
    }
}

fn eval_function_call(state: &mut State, function_call: &ast::FunctionCall) -> Value {
    // Function calls and array element lookups are syntactically equivalent, so both need to be handled here.
    let args = function_call.arguments.iter().map(|expr| eval_expr(state, expr)).collect::<Vec<_>>();

    let function_name = &function_call.function_name;
    if !state.function_table.contains_key(function_name) {
        match state.try_resolve_variable(function_name) {
            Some(&mut VariableTableEntry::Array(ref mut array)) => {
                return array.index(&args).clone();
            },
            _ => panic!("Function or array not found: \"{}\"", function_name)
        }
    }

    let function_table_entry = state.function_table.get(function_name).unwrap();
    match function_table_entry {
        &FunctionTableEntry::Decl(ref function_decl) => {
            let state = unsafe { &mut *(state as *const State as *mut State) }; // Fighting with the borrow checker
            eval_function_decl(state, function_decl, args)
        },
        &FunctionTableEntry::Impl(ref f) => {
            f(&mut state.context, &args)
        }
    }
}

fn eval_function_decl(state: &mut State, function_decl: &ast::FunctionDecl, args: Vec<Value>) -> Value {
    state.push_scope();

    for i in 0..args.len() {
        let function_decl_arg = &function_decl.args[i];
        let name = &function_decl_arg.name;
        let value_type = (&function_decl_arg.type_specifier).into();
        state.add_variable(name.clone(), VariableTableEntry::Variable(Variable {
            name: name.clone(),
            is_const: false,
            value: args[i].cast_to(&value_type),
            value_type: value_type
        }));
    }

    for statement in function_decl.body.iter() {
        interpret_statement(state, statement);
    }

    state.pop_scope();

    Value::Unit // TODO: Proper function return values
}

fn eval_variable_ref(state: &mut State, variable_ref: &ast::VariableRef) -> Value {
    state.resolve_variable(&variable_ref.name).as_variable().value.clone()
}

fn eval_un_op(state: &mut State, un_op: &ast::UnOp) -> Value {
    let arg = eval_expr(state, &un_op.expr);

    let key = (un_op.op.clone(), arg.get_type());
    if let Some(ref un_op_impl) = state.un_op_function_table.get(&key) {
        un_op_impl(&mut state.context, &vec![arg])
    } else {
        panic!("Unrecognized or unsupported un op for key: {:?}", key)
    }
}

fn eval_bin_op(state: &mut State, bin_op: &ast::BinOp) -> Value {
    let lhs = eval_expr(state, &bin_op.lhs);
    let rhs = eval_expr(state, &bin_op.rhs);

    let key = (bin_op.op.clone(), lhs.get_type(), rhs.get_type());
    if let Some(ref bin_op_impl) = state.bin_op_function_table.get(&key) {
        bin_op_impl(&mut state.context, &vec![lhs, rhs])
    } else {
        panic!("Unrecognized or unsupported bin op for key: {:?}", key)
    }
}

fn interpret_statement(state: &mut State, statement: &ast::Statement) {
    match statement {
        &ast::Statement::ArrayDecl(ref array_decl) => interpret_array_decl(state, array_decl),
        &ast::Statement::If(ref if_statement) => interpret_if_statement(state, if_statement),
        &ast::Statement::While(ref while_statement) => interpret_while(state, while_statement),
        &ast::Statement::For(ref for_statement) => interpret_for(state, for_statement),
        &ast::Statement::Restore(ref label_name) => interpret_restore(state, label_name),
        &ast::Statement::Read(ref l_value) => interpret_read(state, l_value),
        &ast::Statement::Assignment(ref assignment) => interpret_assignment(state, assignment),
        &ast::Statement::FunctionCall(ref function_call) => { eval_function_call(state, function_call); },
        _ => panic!("Unrecognized statement: {:?}", statement)
    }
}

fn interpret_array_decl(state: &mut State, array_decl: &ast::ArrayDecl) {
    let dimensions = array_decl.dimensions.iter().map(|expr| eval_expr(state, expr).cast_to_integer().as_integer() + 1).collect::<Vec<_>>();
    let size = dimensions.iter().fold(1, |acc, x| acc * x) as usize;
    let value_type = (&array_decl.type_specifier).into();
    let mut values = Vec::with_capacity(size);
    for _ in 0..size {
        values.push(Value::default(&value_type));
    }
    let array = Array {
        name: array_decl.name.clone(),
        dimensions: dimensions,
        values: values,
        value_type: value_type
    };

    if let Some(variable_table_entry) = state.try_resolve_variable(&array_decl.name) {
        if let &mut VariableTableEntry::Variable(_) = variable_table_entry {
            panic!("Variable was not an array: {}", array_decl.name)
        }
        *variable_table_entry = VariableTableEntry::Array(array);
        return;
    }

    state.add_variable(array_decl.name.clone(), VariableTableEntry::Array(array));
}

fn interpret_if_statement(state: &mut State, if_statement: &ast::If) {
    if eval_expr(state, &if_statement.condition).as_bool() {
        state.push_scope();

        for statement in if_statement.body.iter() {
            interpret_statement(state, statement);
        }

        state.pop_scope();
    } else if let &Some(ref else_clause) = &if_statement.else_clause {
        state.push_scope();

        for statement in else_clause.body.iter() {
            interpret_statement(state, statement);
        }

        state.pop_scope();
    }
}

fn interpret_while(state: &mut State, while_statement: &ast::While) {
    while eval_expr(state, &while_statement.condition).as_bool() {
        state.push_scope();

        for statement in while_statement.body.iter() {
            interpret_statement(state, statement);
        }

        state.pop_scope();
    }
}

fn interpret_for(state: &mut State, for_statement: &ast::For) {
    state.push_scope();

    interpret_assignment(state, &for_statement.initialization);

    let index_l_value = &for_statement.initialization.l_value;
    let conditional = Box::new(ast::Expr::BinOp(ast::BinOp {
        op: ast::Op::Gt,
        lhs: Box::new(ast::Expr::VariableRef(ast::VariableRef {
            name: index_l_value.as_variable_ref().name, // lol
            type_specifier: None
        })),
        rhs: for_statement.to.clone()
    }));
    let step = for_statement.step.clone().map_or(Value::Integer(1), |expr| eval_expr(state, &expr));
    let increment = ast::Statement::Assignment(ast::Assignment {
        l_value: index_l_value.clone(),
        expr: Box::new(ast::Expr::BinOp(ast::BinOp {
            op: ast::Op::Add,
            lhs: Box::new(ast::Expr::VariableRef(index_l_value.as_variable_ref())), // lol
            rhs: step.to_expr()
        }))
    });

    while !eval_expr(state, &conditional).as_bool() {
        for statement in for_statement.body.iter() {
            interpret_statement(state, statement);
        }

        interpret_statement(state, &increment);
    }

    state.pop_scope();
}

fn interpret_restore(state: &mut State, label_name: &String) {
    state.data_pointer = *state.data_labels.get(label_name).unwrap();
}

fn interpret_read(state: &mut State, l_value: &ast::LValue) {
    if state.data_pointer >= state.data_table.len() {
        panic!(
            "Data pointer out of range: {}/{}\nData table: {:#?}",
            state.data_pointer,
            state.data_table.len(),
            state.data_table);
    }
    let value = state.data_table[state.data_pointer].clone();
    state.data_pointer += 1;
    perform_assignment(state, l_value, value);
}

fn perform_assignment(state: &mut State, l_value: &ast::LValue, value: Value) {
    match l_value {
        &ast::LValue::VariableRef(ref variable_ref) => {
            state.add_or_update_variable(&variable_ref.name, value, (&variable_ref.type_specifier).into());
        },
        &ast::LValue::ArrayElemRef(ref array_elem_ref) => {
            let name = &array_elem_ref.array_name;
            let dimensions = array_elem_ref.dimensions.iter().map(|expr| eval_expr(state, &expr)).collect::<Vec<_>>();
            state.update_array_elem_ref(name, dimensions, value);
        }
    }
}

fn interpret_assignment(state: &mut State, assignment: &ast::Assignment) {
    let value = eval_expr(state, &assignment.expr);
    perform_assignment(state, &assignment.l_value, value);
}
