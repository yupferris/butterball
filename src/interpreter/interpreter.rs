use super::super::ast;
use super::context::*;

pub fn interpret(ast: &ast::Root) {
    let mut context = Context::new(ast);
    for node in ast.nodes.iter() {
        interpret_node(&mut context, node);
    }
}

fn interpret_node(context: &mut Context, node: &ast::Node) {
    match node {
        &ast::Node::GlobalVariableDecl(ref variable_decl) => interpret_variable_decl(context, variable_decl),
        &ast::Node::ConstDecl(ref const_decl) => interpret_const_decl(context, const_decl),
        &ast::Node::Statement(ref statement) => interpret_statement(context, statement),
        _ => panic!("Unrecognized node: {:?}", node)
    }
}

fn interpret_variable_decl(context: &mut Context, variable_decl: &ast::VariableDecl) {
    let value = variable_decl.init_expr.clone().map_or(Value::default(&variable_decl.type_specifier), |expr| eval_expr(context, &expr));
    context.add_variable(variable_decl.name.clone(), VariableTableEntry::Variable(Variable {
        name: variable_decl.name.clone(),
        is_const: false,
        value: value
    }));
}

fn interpret_const_decl(context: &mut Context, const_decl: &ast::ConstDecl) {
    let value = eval_expr(context, &const_decl.init_expr);
    context.add_variable(const_decl.name.clone(), VariableTableEntry::Variable(Variable {
        name: const_decl.name.clone(),
        is_const: true,
        value: value
    }));
}

fn eval_expr(context: &mut Context, expr: &Box<ast::Expr>) -> Value {
    match **expr {
        ast::Expr::FloatLiteral(value) => Value::Float(value),
        ast::Expr::IntegerLiteral(value) => Value::Integer(value),
        ast::Expr::BoolLiteral(value) => Value::Bool(value),
        ast::Expr::StringLiteral(ref value) => Value::String(value.clone()),
        ast::Expr::FunctionCall(ref function_call) => eval_function_call(context, function_call),
        ast::Expr::VariableRef(ref variable_ref) => eval_variable_ref(context, variable_ref),
        ast::Expr::BinOp(ref bin_op) => eval_bin_op(context, bin_op),
        _ => panic!("Unrecognized expression: {:?}", expr)
    }
}

fn eval_function_call(context: &mut Context, function_call: &ast::FunctionCall) -> Value {
    let function_name = &function_call.function_name;
    if !context.function_table.contains_key(function_name) {
        panic!("Function not found: \"{}\"", function_name);
    }
    let function_table_entry = context.function_table.get(function_name).unwrap();
    match function_table_entry {
        &FunctionTableEntry::Decl(ref function_decl) => {
            let context = unsafe { &mut *(context as *const Context as *mut Context) }; // Fuck you, borrow checker!
            let args = function_call.arguments.iter().map(|expr| eval_expr(context, expr)).collect::<Vec<_>>();
            eval_function_decl(context, function_decl, args)
        },
        &FunctionTableEntry::Impl(ref f) => {
            let context = unsafe { &mut *(context as *const Context as *mut Context) }; // Fuck you, borrow checker!
            let args = function_call.arguments.iter().map(|expr| eval_expr(context, expr)).collect::<Vec<_>>();
            f(context, &args)
        }
    }
}

fn eval_function_decl(context: &mut Context, function_decl: &ast::FunctionDecl, args: Vec<Value>) -> Value {
    context.push_variable_frame();

    for i in 0..args.len() {
        let name = &function_decl.args[i].name;
        context.add_variable(name.clone(), VariableTableEntry::Variable(Variable {
            name: name.clone(),
            is_const: false,
            value: args[i].clone()
        }));
    }

    for statement in function_decl.body.iter() {
        interpret_statement(context, statement);
    }

    context.pop_variable_frame();

    Value::Unit // TODO: Proper function return values
}

fn eval_variable_ref(context: &mut Context, variable_ref: &ast::VariableRef) -> Value {
    context.resolve_variable(&variable_ref.name).as_variable().value.clone()
}

fn eval_bin_op(context: &mut Context, bin_op: &ast::BinOp) -> Value {
    let lhs = eval_expr(context, &bin_op.lhs);
    let rhs = eval_expr(context, &bin_op.rhs);
    match bin_op.op {
        ast::Op::Div =>
            if lhs.is_integer() && rhs.is_integer() {
                Value::Integer(lhs.as_integer() / rhs.as_integer())
            } else {
                panic!("Invalid values for divide: {:?}, {:?}", lhs, rhs)
            },
        _ => panic!("Unrecognized or unsupported bin op: {:?}", bin_op.op)
    }
}

fn interpret_statement(context: &mut Context, statement: &ast::Statement) {
    match statement {
        &ast::Statement::ArrayDecl(ref array_decl) => interpret_array_decl(context, array_decl),
        &ast::Statement::FunctionCall(ref function_call) => { eval_function_call(context, function_call); },
        _ => panic!("Unrecognized statement: {:?}", statement)
    }
}

fn interpret_array_decl(context: &mut Context, array_decl: &ast::ArrayDecl) {
    let dimensions = array_decl.dimensions.iter().map(|expr| eval_expr(context, expr).as_integer()).collect::<Vec<_>>();
    let size = dimensions.iter().fold(0, |acc, x| acc + x) as usize;
    let mut values = Vec::with_capacity(size);
    for _ in 0..size {
        values.push(Value::default(&array_decl.type_specifier));
    }
    let array = Array {
        name: array_decl.name.clone(),
        dimensions: dimensions,
        values: values
    };
    // TODO: Make sure to resize an array if it exists already
    context.add_variable(array_decl.name.clone(), VariableTableEntry::Array(array));
}
