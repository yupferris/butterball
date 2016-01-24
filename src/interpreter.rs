use super::ast;

use std::collections::HashMap;

pub fn interpret(ast: &ast::Root) {
    let mut context = Context::new(ast);
    for node in ast.nodes.iter() {
        interpret_node(&mut context, node);
    }
}

struct Context {
    function_table: FunctionTable,

    program_state: ProgramState
}

impl Context {
    fn new(ast: &ast::Root) -> Context {
        Context {
            function_table: build_function_table(ast),

            program_state: ProgramState::default()
        }
    }
}

type FunctionTable = HashMap<String, FunctionTableEntry>;

enum FunctionTableEntry {
    Decl(ast::FunctionDecl),
    Impl(Box<Fn(&mut Context, &Vec<Value>) -> Value>)
}

#[derive(Default)]
struct ProgramState {
    app_title: String
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

    ret.insert(String::from("AppTitle"), FunctionTableEntry::Impl(Box::new(|context, args| {
        context.program_state.app_title = args[0].as_string();
        println!("New app title: \"{}\"", context.program_state.app_title);

        Value::Unit
    })));

    ret
}

#[derive(Debug)]
enum Value {
    Unit,
    String(String)
}

impl Value {
    fn as_string(&self) -> String {
        match self {
            &Value::String(ref value) => value.clone(),
            _ => panic!("Value was not a string: {:?}", self)
        }
    }
}

fn interpret_node(context: &mut Context, node: &ast::Node) {
    match node {
        &ast::Node::Statement(ref statement) => interpret_statement(context, statement),
        _ => panic!("Unrecognized node: {:?}", node)
    }
}

fn interpret_statement(context: &mut Context, statement: &ast::Statement) {
    match statement {
        &ast::Statement::FunctionCall(ref function_call) => interpret_function_call(context, function_call),
        _ => panic!("Unrecognized statement: {:?}", statement)
    }
}

fn interpret_function_call(context: &mut Context, function_call: &ast::FunctionCall) {
    let function_name = &function_call.function_name;
    if !context.function_table.contains_key(function_name) {
        panic!("Function not found: \"{}\"", function_name);
    }
    let function_table_entry = context.function_table.get(function_name).unwrap();
    match function_table_entry {
        &FunctionTableEntry::Impl(ref f) => {
            let context = unsafe { &mut *(context as *const Context as *mut Context) }; // Fuck you, borrow checker!
            let args = function_call.arguments.iter().map(|expr| eval_expr(context, expr)).collect::<Vec<_>>();
            f(context, &args);
        },
        _ => panic!("Unrecognized function table entry")
    }
}

fn eval_expr(context: &mut Context, expr: &Box<ast::Expr>) -> Value {
    match **expr {
        ast::Expr::StringLiteral(ref value) => Value::String(value.clone()),
        _ => panic!("Unrecognized expression: {:?}", expr)
    }
}
