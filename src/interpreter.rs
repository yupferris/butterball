use super::il;
use super::value::*;
use super::context::*;

pub fn interpret(program: &il::Program) {
    let mut state = State::new(program);
    interpret_function(&program.main_function, program, &mut state);
}

struct State {
    context: Context,

    globals: Vec<Variable>,

    stack: Vec<Value>
}

impl State {
    fn new(program: &il::Program) -> State {
        State {
            context: Context::new(),

            globals: program.globals.iter()
                .map(|x| match x {
                    &il::Variable::SingleVariable(ref single_variable) =>
                        Variable::SingleVariable(SingleVariable { value: Value::default(&single_variable.value_type) }),
                    &il::Variable::Array(_) =>
                        Variable::Array(Array { dimensions: Vec::new(), values: Vec::new() })
                })
                .collect::<Vec<_>>(),

            stack: Vec::new()
        }
    }
}

#[derive(Debug)]
enum Variable {
    SingleVariable(SingleVariable),
    Array(Array)
}

#[derive(Debug)]
struct SingleVariable {
    value: Value
}

#[derive(Debug)]
struct Array {
    dimensions: Vec<i32>,
    values: Vec<Value>
}

fn interpret_function(function: &il::Function, program: &il::Program, state: &mut State) {
    for statement in function.body.iter() {
        interpret_statement(statement, program, state)
    }
}

fn interpret_statement(statement: &il::Statement, program: &il::Program, state: &mut State) {
    match statement {
        &il::Statement::ArrayAlloc(ref array_alloc) => interpret_array_alloc(array_alloc, program, state),
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
        values: values
    };

    let var = match &array_alloc.array_ref {
        &il::ArrayRef::Global(index) => &mut state.globals[index]
    };
    match var {
        &mut Variable::Array(ref mut array_var) => { *array_var = array; },
        _ => panic!("Variable was not an array: {:#?}", var)
    }
}

fn interpret_assignment(assignment: &il::Assignment, program: &il::Program, state: &mut State) {
    let value = eval_expr(&assignment.expr, program, state);
    match &assignment.l_value {
        &il::LValue::VariableRef(ref variable_ref) => {
            match variable_ref {
                &il::VariableRef::Global(index) => {
                    let variable = &mut state.globals[index];
                    match variable {
                        &mut Variable::SingleVariable(ref mut single_variable) => { single_variable.value = value; },
                        _ => panic!("Tried to assign to an array: {:#?}", assignment)
                    }
                },
                _ => panic!("Local variable l values not yet supported")
            }
        },
        _ => panic!("Unrecognized l value: {:#?}", assignment.l_value)
    }
}

fn eval_function_call(function_call: &il::FunctionCall, program: &il::Program, state: &mut State) -> Value {
    // TODO: Use stack for arguments to avoid vector allocation
    let arguments =
        function_call.arguments.iter()
        .map(|x| eval_expr(x, program, state))
        .collect::<Vec<_>>();
    let function_table_entry = &program.function_table[function_call.function_index];
    match function_table_entry {
        &il::FunctionTableEntry::FunctionImpl(ref function_impl) => (function_impl.function)(&mut state.context, &arguments),
        _ => panic!("Unrecognized function table entry: {:#?}", function_table_entry)
    }
}

fn eval_expr(expr: &il::Expr, program: &il::Program, state: &mut State) -> Value {
    match expr {
        &il::Expr::Integer(value) => Value::Integer(value),
        &il::Expr::String(ref value) => Value::String(value.clone()),
        &il::Expr::FunctionCall(ref function_call) => eval_function_call(function_call, program, state),
        &il::Expr::VariableRef(ref variable_ref) => eval_variable_ref(variable_ref, state),
        _ => panic!("Unrecognized expression: {:#?}", expr)
    }
}

fn eval_variable_ref(variable_ref: &il::VariableRef, state: &mut State) -> Value {
    match variable_ref {
        &il::VariableRef::Global(index) => {
            let variable = &state.globals[index];
            match variable {
                &Variable::SingleVariable(ref single_variable) => single_variable.value.clone(),
                _ => panic!("Variable was an array: {:#?}", variable_ref)
            }
        },
        _ => panic!("Unsupported variable ref: {:#?}", variable_ref)
    }
}
