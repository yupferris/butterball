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
        ast::Expr::UnOp(ref un_op) => eval_un_op(context, un_op),
        ast::Expr::BinOp(ref bin_op) => eval_bin_op(context, bin_op)
    }
}

fn eval_function_call(context: &mut Context, function_call: &ast::FunctionCall) -> Value {
    // Function calls and array element lookups are syntactically equivalent, so both need to be handled here.
    let args = function_call.arguments.iter().map(|expr| eval_expr(context, expr)).collect::<Vec<_>>();

    let function_name = &function_call.function_name;
    if !context.function_table.contains_key(function_name) {
        match context.try_resolve_variable(function_name) {
            Some(&mut VariableTableEntry::Array(ref mut array)) => {
                return array.index(&args).clone();
            },
            _ => panic!("Function or array not found: \"{}\"", function_name)
        }
    }

    let function_table_entry = context.function_table.get(function_name).unwrap();
    match function_table_entry {
        &FunctionTableEntry::Decl(ref function_decl) => {
            let context = unsafe { &mut *(context as *const Context as *mut Context) }; // Fuck you, borrow checker!
            eval_function_decl(context, function_decl, args)
        },
        &FunctionTableEntry::Impl(ref f) => {
            let context = unsafe { &mut *(context as *const Context as *mut Context) }; // Fuck you, borrow checker!
            f(context, &args)
        }
    }
}

fn eval_function_decl(context: &mut Context, function_decl: &ast::FunctionDecl, args: Vec<Value>) -> Value {
    context.push_scope();

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

    context.pop_scope();

    Value::Unit // TODO: Proper function return values
}

fn eval_variable_ref(context: &mut Context, variable_ref: &ast::VariableRef) -> Value {
    context.resolve_variable(&variable_ref.name).as_variable().value.clone()
}

fn eval_un_op(context: &mut Context, un_op: &ast::UnOp) -> Value {
    let arg = eval_expr(context, &un_op.expr);

    let key = (un_op.op.clone(), arg.get_type());
    if let Some(ref un_op_impl) = context.un_op_function_table.get(&key) {
        let context = unsafe { &mut *(context as *const Context as *mut Context) }; // Fuck you, borrow checker!
        un_op_impl(context, &vec![arg])
    } else {
        panic!("Unrecognized or unsupported un op for key: {:?}", key)
    }
}

fn eval_bin_op(context: &mut Context, bin_op: &ast::BinOp) -> Value {
    let lhs = eval_expr(context, &bin_op.lhs);
    let rhs = eval_expr(context, &bin_op.rhs);

    let key = (bin_op.op.clone(), lhs.get_type(), rhs.get_type());
    if let Some(ref bin_op_impl) = context.bin_op_function_table.get(&key) {
        let context = unsafe { &mut *(context as *const Context as *mut Context) }; // Fuck you, borrow checker!
        bin_op_impl(context, &vec![lhs, rhs])
    } else {
        panic!("Unrecognized or unsupported bin op for key: {:?}", key)
    }
}

fn interpret_statement(context: &mut Context, statement: &ast::Statement) {
    match statement {
        &ast::Statement::ArrayDecl(ref array_decl) => interpret_array_decl(context, array_decl),
        &ast::Statement::If(ref if_statement) => interpret_if_statement(context, if_statement),
        &ast::Statement::While(ref while_statement) => interpret_while(context, while_statement),
        &ast::Statement::For(ref for_statement) => interpret_for(context, for_statement),
        &ast::Statement::Restore(ref label_name) => interpret_restore(context, label_name),
        &ast::Statement::Read(ref l_value) => interpret_read(context, l_value),
        &ast::Statement::Assignment(ref assignment) => interpret_assignment(context, assignment),
        &ast::Statement::FunctionCall(ref function_call) => { eval_function_call(context, function_call); },
        _ => panic!("Unrecognized statement: {:?}", statement)
    }
}

fn interpret_array_decl(context: &mut Context, array_decl: &ast::ArrayDecl) {
    let dimensions = array_decl.dimensions.iter().map(|expr| eval_expr(context, expr).as_integer() + 1).collect::<Vec<_>>();
    let size = dimensions.iter().fold(1, |acc, x| acc * x) as usize;
    let mut values = Vec::with_capacity(size);
    for _ in 0..size {
        values.push(Value::default(&array_decl.type_specifier));
    }
    let array = Array {
        name: array_decl.name.clone(),
        dimensions: dimensions,
        values: values
    };

    if let Some(variable_table_entry) = context.try_resolve_variable(&array_decl.name) {
        if let &mut VariableTableEntry::Variable(_) = variable_table_entry {
            panic!("Variable was not an array: {}", array_decl.name)
        }
        *variable_table_entry = VariableTableEntry::Array(array);
        return;
    }

    context.add_variable(array_decl.name.clone(), VariableTableEntry::Array(array));
}

fn interpret_if_statement(context: &mut Context, if_statement: &ast::If) {
    if eval_expr(context, &if_statement.condition).as_bool() {
        context.push_scope();

        for statement in if_statement.body.iter() {
            interpret_statement(context, statement);
        }

        context.pop_scope();
    } else if let &Some(ref else_clause) = &if_statement.else_clause {
        context.push_scope();

        for statement in else_clause.body.iter() {
            interpret_statement(context, statement);
        }

        context.pop_scope();
    }
}

fn interpret_while(context: &mut Context, while_statement: &ast::While) {
    while eval_expr(context, &while_statement.condition).as_bool() {
        context.push_scope();

        for statement in while_statement.body.iter() {
            interpret_statement(context, statement);
        }

        context.pop_scope();
    }
}

fn interpret_for(context: &mut Context, for_statement: &ast::For) {
    context.push_scope();

    interpret_assignment(context, &for_statement.initialization);

    let index_l_value = &for_statement.initialization.l_value;
    let step = for_statement.step.clone().map_or(Value::Integer(1), |expr| eval_expr(context, &expr));
    let increment = ast::Statement::Assignment(ast::Assignment {
        l_value: index_l_value.clone(),
        expr: Box::new(ast::Expr::BinOp(ast::BinOp {
            op: ast::Op::Add,
            lhs: Box::new(ast::Expr::VariableRef(index_l_value.as_variable_ref())), // lol
            rhs: step.to_expr()
        }))
    });

    loop {
        let to = eval_expr(context, &for_statement.to);
        let conditional = Box::new(ast::Expr::BinOp(ast::BinOp {
            op: ast::Op::Gt,
            lhs: Box::new(ast::Expr::VariableRef(ast::VariableRef {
                name: index_l_value.as_variable_ref().name, // lol
                type_specifier: None
            })),
            rhs: to.to_expr()
        }));
        if eval_expr(context, &conditional).as_bool() {
            break;
        }

        context.push_scope();

        for statement in for_statement.body.iter() {
            interpret_statement(context, statement);
        }

        context.pop_scope();

        interpret_statement(context, &increment);
    }

    context.pop_scope();
}

fn interpret_restore(context: &mut Context, label_name: &String) {
    context.program_state.data_pointer = *context.data_labels.get(label_name).unwrap();
}

fn interpret_read(context: &mut Context, l_value: &ast::LValue) {
    if context.program_state.data_pointer >= context.data_table.len() {
        panic!(
            "Data pointer out of range: {}/{}\nData table: {:#?}",
            context.program_state.data_pointer,
            context.data_table.len(),
            context.data_table);
    }
    let expr = context.data_table[context.program_state.data_pointer].to_expr();
    context.program_state.data_pointer += 1;
    interpret_assignment(context, &ast::Assignment {
        l_value: l_value.clone(),
        expr: expr
    });
}

fn interpret_assignment(context: &mut Context, assignment: &ast::Assignment) {
    let value = eval_expr(context, &assignment.expr);
    match assignment.l_value {
        ast::LValue::VariableRef(ref variable_ref) => {
            let name = &variable_ref.name;
            context.add_or_update_variable(name, value);
        },
        ast::LValue::ArrayElemRef(ref array_elem_ref) => {
            let name = &array_elem_ref.array_name;
            let dimensions = array_elem_ref.dimensions.iter().map(|expr| eval_expr(context, &expr)).collect::<Vec<_>>();
            context.update_array_elem_ref(name, dimensions, value);
        }
    }
}
