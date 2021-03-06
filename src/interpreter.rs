use il;
use value::*;
use stdlib::context::*;

use std::ops::{Index, IndexMut};

const STACK_SIZE: usize = 4 * 1024 * 1024;

pub fn interpret(program: &il::Program) {
    let mut state = State::new(program);
    interpret_function(&program.main_function, program, &mut state);
}

struct State {
    context: Context,

    globals: Vec<Variable>,

    // TODO: Nest this part of the structure?
    stack: Stack,
    base_pointer: usize,

    data_pointer: usize
}

impl State {
    fn new(program: &il::Program) -> State {
        State {
            context: Context::new(),

            globals: program.globals.iter()
                .map(|x| match x {
                    &il::Variable::SingleVariable(ref single_variable) =>
                        Variable::SingleVariable(SingleVariable {
                            value: Value::default(&single_variable.value_type),
                            value_type: single_variable.value_type
                        }),
                    &il::Variable::Array(ref array) =>
                        Variable::Array(Array {
                            dimensions: Vec::new(),
                            values: Vec::new(),
                            value_type: array.value_type
                        })
                })
                .collect::<Vec<_>>(),

            stack: Stack::new(),
            base_pointer: 0,

            data_pointer: 0
        }
    }
}

struct Stack {
    data: Box<[Value]>,
    position: usize
}

impl Stack {
    fn new() -> Stack {
        Stack {
            data: vec![Value::Unit; STACK_SIZE].into_boxed_slice(),
            position: 0
        }
    }

    fn push(&mut self, value: Value) {
        let index = self.position;
        self[index] = value;
        self.position += 1;
    }
}

impl Index<usize> for Stack {
    type Output = Value;

    fn index(&self, index: usize) -> &Value {
        &self.data[index]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, index: usize) -> &mut Value {
        &mut self.data[index]
    }
}

#[derive(Debug)]
enum Variable {
    SingleVariable(SingleVariable),
    Array(Array)
}

#[derive(Debug)]
struct SingleVariable {
    value: Value,
    value_type: ValueType
}

#[derive(Debug)]
struct Array {
    dimensions: Vec<i32>,
    values: Vec<Value>,
    value_type: ValueType
}

fn interpret_function(function: &il::Function, program: &il::Program, state: &mut State) {
    for statement in function.body.iter() {
        interpret_statement(statement, program, state)
    }
}

fn interpret_statement(statement: &il::Statement, program: &il::Program, state: &mut State) {
    match statement {
        &il::Statement::ArrayAlloc(ref array_alloc) => interpret_array_alloc(array_alloc, program, state),
        &il::Statement::If(ref if_statement) => interpret_if_statement(if_statement, program, state),
        &il::Statement::While(ref while_statement) => interpret_while_statement(while_statement, program, state),
        &il::Statement::For(ref for_statement) => interpret_for_statement(for_statement, program, state),
        &il::Statement::Restore(index) => { state.data_pointer = index; },
        &il::Statement::Read(ref l_value) => interpret_read(l_value, program, state),
        &il::Statement::Assignment(ref assignment) => interpret_assignment(assignment, program, state),
        &il::Statement::FunctionCall(ref function_call) => { eval_function_call(function_call, program, state); },
        _ => panic!("Unrecognized statement: {:#?}", statement)
    }
}

fn interpret_array_alloc(array_alloc: &il::ArrayAlloc, program: &il::Program, state: &mut State) {
    let dimensions =
        array_alloc.dimensions.iter()
        .map(|expr| eval_expr(expr, program, state).cast_to_integer().as_integer() + 1)
        .collect::<Vec<_>>();
    let size =
        dimensions.iter()
        .fold(1, |acc, x| acc * x) as usize;
    let mut values = Vec::with_capacity(size);
    for _ in 0..size {
        values.push(Value::default(&array_alloc.value_type));
    }
    let array = Array {
        dimensions: dimensions,
        values: values,
        value_type: array_alloc.value_type
    };

    let var = match &array_alloc.array_ref {
        &il::ArrayRef::Global(index) => &mut state.globals[index]
    };
    match var {
        &mut Variable::Array(ref mut array_var) => { *array_var = array; },
        _ => panic!("Variable was not an array: {:#?}", var)
    }
}

fn interpret_if_statement(if_statement: &il::If, program: &il::Program, state: &mut State) {
    if eval_expr(&if_statement.condition, program, state).as_bool() {
        for statement in if_statement.body.iter() {
            interpret_statement(statement, program, state);
        }
    } else {
        if let &Some(ref else_clause) = &if_statement.else_clause {
            for statement in else_clause.iter() {
                interpret_statement(statement, program, state);
            }
        }
    }
}

fn interpret_while_statement(while_statement: &il::While, program: &il::Program, state: &mut State) {
    while eval_expr(&while_statement.condition, program, state).as_bool() {
        for statement in while_statement.body.iter() {
            interpret_statement(statement, program, state);
        }
    }
}

fn interpret_for_statement(for_statement: &il::For, program: &il::Program, state: &mut State) {
    interpret_assignment(&for_statement.initialization, program, state);
    while eval_expr(&for_statement.condition, program, state).as_bool() {
        for statement in for_statement.body.iter() {
            interpret_statement(statement, program, state);
        }
        interpret_assignment(&for_statement.increment, program, state);
    }
}

fn interpret_read(l_value: &il::LValue, program: &il::Program, state: &mut State) {
    if state.data_pointer >= program.data_table.len() {
        panic!(
            "Data pointer out of range: {}/{}\nData table: {:#?}",
            state.data_pointer,
            program.data_table.len(),
            program.data_table);
    }
    let value = program.data_table[state.data_pointer].clone();
    state.data_pointer += 1;
    perform_assignment(l_value, value, program, state);
}

fn interpret_assignment(assignment: &il::Assignment, program: &il::Program, state: &mut State) {
    let value = eval_expr(&assignment.expr, program, state);
    perform_assignment(&assignment.l_value, value, program, state);
}

fn perform_assignment(l_value: &il::LValue, value: Value, program: &il::Program, state: &mut State) {
    match l_value {
        &il::LValue::VariableRef(ref variable_ref) => {
            match variable_ref {
                &il::VariableRef::Global(ref global_variable_ref) => {
                    match &mut state.globals[global_variable_ref.global_index] {
                        &mut Variable::SingleVariable(ref mut single_variable) => {
                            single_variable.value = value;
                        },
                        _ => panic!("LValue was not a single variable: {:#?}", l_value)
                    }
                },
                &il::VariableRef::Local(ref local_variable_ref) => {
                    // TODO: Support local arrays
                    state.stack[state.base_pointer + local_variable_ref.local_index] = value;
                }
            }
        },
        &il::LValue::ArrayElemRef(ref array_elem_ref) => {
            *index_array_elem_ref(array_elem_ref, program, state) = value;
        }
    }
}

fn index_array_elem_ref<'a>(array_elem_ref: &il::ArrayElemRef, program: &il::Program, state: &'a mut State) -> &'a mut Value {
    match array_elem_ref {
        &il::ArrayElemRef::Global(ref global_array_elem_ref) => {
            let old_stack_pointer = state.stack.position;

            for expr in global_array_elem_ref.dimensions.iter() {
                let value = eval_expr(expr, program, state);
                state.stack.push(value);
            }

            let ret = match &mut state.globals[global_array_elem_ref.global_index] {
                &mut Variable::Array(ref mut array) => {
                    let mut index = 0;
                    let mut dim_multiplier = 1;
                    for (i, current_dimension_size) in array.dimensions.iter().enumerate().rev() {
                        index += state.stack[old_stack_pointer + i].cast_to_integer().as_integer() * dim_multiplier;
                        dim_multiplier *= *current_dimension_size;
                    }
                    &mut array.values[index as usize]
                },
                _ => panic!("Variable was not an array: {:#?}", array_elem_ref)
            };

            state.stack.position = old_stack_pointer;

            ret
        }
    }
}

fn eval_function_call(function_call: &il::FunctionCall, program: &il::Program, state: &mut State) -> Value {
    let function_table_entry = &program.function_table[function_call.function_index];
    match function_table_entry {
        &il::FunctionTableEntry::Function(ref function) => {
            //println!("Evaluating function: {}", function.signature.name);

            let caller_base_pointer = state.base_pointer;
            let callee_base_pointer = state.stack.position;

            // TODO: Remove casts when proper arg types are ensured by the compiler
            for (expr, arg) in function_call.arguments.iter().zip(function.signature.args.iter()) {
                let value = eval_expr(expr, program, state).cast_to(&arg.value_type);
                state.stack.push(value);
            }

            state.stack.position += function.stack_frame_size - function_call.arguments.len();

            /*println!("---- current stack frame ----");
            for i in state.base_pointer..state.stack.position {
                println!("{:#?}", state.stack[i]);
            }
            println!("");*/

            // This assignment has to happen _after_ evaluating the function arguments, otherwise arguments that reference the caller's locals
            // will be indexed relative to the callee's stack frame, rather than the caller's
            state.base_pointer = callee_base_pointer;

            interpret_function(function, program, state);

            state.stack.position = callee_base_pointer;
            state.base_pointer = caller_base_pointer;

            // TODO: Proper return types
            Value::Unit
        },
        &il::FunctionTableEntry::FunctionImpl(ref function_impl) => {
            let callee_base_pointer = state.stack.position;

            for arg in function_call.arguments.iter() {
                let value = eval_expr(arg, program, state);
                state.stack.push(value);
            }

            let ret = (function_impl.function)(&mut state.context, &state.stack.data[callee_base_pointer..state.stack.position]);

            state.stack.position = callee_base_pointer;

            ret
        }
    }
}

fn eval_expr(expr: &il::Expr, program: &il::Program, state: &mut State) -> Value {
    match expr {
        &il::Expr::Integer(value) => Value::Integer(value),
        &il::Expr::Float(value) => Value::Float(value),
        &il::Expr::Bool(value) => Value::Bool(value),
        &il::Expr::String(ref value) => Value::String(value.clone()),
        &il::Expr::Cast(ref cast) => eval_expr(&cast.expr, program, state).cast_to(&cast.target_type),
        &il::Expr::FunctionCall(ref function_call) => eval_function_call(function_call, program, state),
        &il::Expr::ArrayElemRef(ref array_elem_ref) => index_array_elem_ref(array_elem_ref, program, state).clone(),
        &il::Expr::VariableRef(ref variable_ref) => eval_variable_ref(variable_ref, state),
        &il::Expr::UnOp(ref un_op) => eval_un_op(un_op, program, state),
        &il::Expr::BinOp(ref bin_op) => eval_bin_op(bin_op, program, state)
    }
}

fn eval_variable_ref(variable_ref: &il::VariableRef, state: &mut State) -> Value {
    match variable_ref {
        &il::VariableRef::Global(ref global_variable_ref) => {
            let variable = &state.globals[global_variable_ref.global_index];
            match variable {
                &Variable::SingleVariable(ref single_variable) => single_variable.value.clone(),
                _ => panic!("Variable was an array: {:#?}", variable_ref)
            }
        },
        &il::VariableRef::Local(ref local_variable_ref) => state.stack[state.base_pointer as usize + local_variable_ref.local_index].clone()
    }
}

fn eval_un_op(un_op: &il::UnOp, program: &il::Program, state: &mut State) -> Value {
    let arg = eval_expr(&un_op.expr, program, state);

    let callee_base_pointer = state.stack.position;

    state.stack.push(arg);

    let ret = (program.un_op_impls_table[un_op.impl_index].function)(
        &mut state.context, &state.stack.data[callee_base_pointer..state.stack.position]);

    state.stack.position = callee_base_pointer;

    ret
}

fn eval_bin_op(bin_op: &il::BinOp, program: &il::Program, state: &mut State) -> Value {
    let lhs_value = eval_expr(&bin_op.lhs, program, state);
    let rhs_value = eval_expr(&bin_op.rhs, program, state);

    let callee_base_pointer = state.stack.position;

    state.stack.push(lhs_value);
    state.stack.push(rhs_value);

    let ret = (program.bin_op_impls_table[bin_op.impl_index].function)(
        &mut state.context, &state.stack.data[callee_base_pointer..state.stack.position]);

    state.stack.position = callee_base_pointer;

    ret
}
