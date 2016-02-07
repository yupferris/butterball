use value::*;
use ast;
use il;
use impls;

use std::collections::HashMap;

// TODO: Proper error handling?
pub fn compile(root: &ast::Root) -> il::Program {
    let un_op_pairs = impls::build_un_op_impls_table();
    let un_op_index_map =
        un_op_pairs.iter()
        .enumerate()
        .map(|(i, x)| (x.0.clone(), i))
        .collect::<HashMap<_, _>>();
    let un_op_impls_table =
        un_op_pairs.into_iter()
        .map(|x| x.1)
        .collect::<Vec<_>>();

    let bin_op_pairs = impls::build_bin_op_impls_table();
    let bin_op_index_map =
        bin_op_pairs.iter()
        .enumerate()
        .map(|(i, x)| (x.0.clone(), i))
        .collect::<HashMap<_, _>>();
    let bin_op_impls_table =
        bin_op_pairs.into_iter()
        .map(|x| x.1)
        .collect::<Vec<_>>();

    let globals = compile_globals(root);
    let globals_index_map =
        globals.iter()
        .enumerate()
        .map(|(i, x)| {
            let name = match x {
                &il::Variable::SingleVariable(ref single_variable) => single_variable.name.clone(),
                &il::Variable::Array(ref array) => array.name.clone()
            };
            (name, i)
        })
        .collect::<HashMap<_, _>>();

    let (data_table, data_labels_map) = build_data_table_and_labels_map(root);

    let function_impls = impls::build_impls_table();

    let function_asts = get_function_asts(root);
    let function_index_map =
        function_asts.iter().map(|function_ast| function_ast.name.clone())
        .chain(function_impls.iter().map(|function_impl| function_impl.name.clone()))
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect::<HashMap<_, _>>();

    let function_types_table =
        function_asts.iter()
        .map(|_| ValueType::Unit)
        .chain(function_impls.iter().map(|x| x.return_type))
        .collect::<Vec<_>>();

    let function_table =
        function_asts.iter()
        .map(|x| compile_function(
            x,
            &un_op_index_map,
            &un_op_impls_table,
            &bin_op_index_map,
            &bin_op_impls_table,
            &globals,
            &globals_index_map,
            &data_labels_map,
            &function_index_map,
            &function_types_table))
        .map(|x| il::FunctionTableEntry::Function(x))
        .chain(function_impls.into_iter().map(|x| il::FunctionTableEntry::FunctionImpl(x)))
        .collect::<Vec<_>>();

    // We build a main function as an AST node so we can compile it like any other function
    let main_function_ast = build_main_function_ast(root);
    let main_function =
        compile_function(
            &main_function_ast,
            &un_op_index_map,
            &un_op_impls_table,
            &bin_op_index_map,
            &bin_op_impls_table,
            &globals,
            &globals_index_map,
            &data_labels_map,
            &function_index_map,
            &function_types_table);

    il::Program {
        un_op_impls_table: un_op_impls_table,
        bin_op_impls_table: bin_op_impls_table,
        globals: globals,
        data_table: data_table,
        function_table: function_table,
        main_function: main_function
    }
}

fn compile_globals(root: &ast::Root) -> Vec<il::Variable> {
    root.nodes.iter()
        .filter_map(|node| match node {
            &ast::Node::GlobalVariableDecl(ref variable_decl) => Some(compile_variable(variable_decl)),
            &ast::Node::ConstDecl(ref const_decl) => Some(compile_const(const_decl)),
            &ast::Node::Statement(ref statement) =>
                match statement {
                    &ast::Statement::ArrayDecl(ref array_decl) => Some(compile_array_variable(array_decl)),
                    _ => None
                },
            _ => None
        })
        .collect::<Vec<_>>()
}

fn compile_variable(variable_decl: &ast::VariableDecl) -> il::Variable {
    il::Variable::SingleVariable(compile_single_variable(variable_decl))
}

fn compile_single_variable(variable_decl: &ast::VariableDecl) -> il::SingleVariable {
    il::SingleVariable {
        name: variable_decl.name.clone(),
        is_const: false,
        value_type: compile_value_type(&variable_decl.type_specifier)
    }
}

fn compile_value_type(type_specifier: &Option<ast::TypeSpecifier>) -> ValueType {
    match type_specifier {
        &Some(ast::TypeSpecifier::Int) | &None => ValueType::Integer,
        &Some(ast::TypeSpecifier::Float) => ValueType::Float,
        &Some(ast::TypeSpecifier::String) => ValueType::String,
        _ => panic!("Unrecognized type specifier: {:?}", type_specifier)
    }
}

fn compile_const(const_decl: &ast::ConstDecl) -> il::Variable {
    il::Variable::SingleVariable(il::SingleVariable {
        name: const_decl.name.clone(),
        is_const: true,
        value_type: compile_value_type(&const_decl.type_specifier)
    })
}

// TODO: Better name?
fn compile_array_variable(array_decl: &ast::ArrayDecl) -> il::Variable {
    il::Variable::Array(il::Array {
        name: array_decl.name.clone(),
        value_type: compile_value_type(&array_decl.type_specifier)
    })
}

fn build_data_table_and_labels_map(root: &ast::Root) -> (Vec<Value>, HashMap<String, usize>) {
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

fn get_function_asts(root: &ast::Root) -> Vec<ast::FunctionDecl> {
    root.nodes.iter()
        .filter_map(|node| match node {
            &ast::Node::FunctionDecl(ref function_decl) => Some(function_decl.clone()),
            _ => None
        })
        .collect::<Vec<_>>()
}

fn build_main_function_ast(root: &ast::Root) -> ast::FunctionDecl {
    ast::FunctionDecl {
        name: String::from("$main"),
        type_specifier: None,
        args: Vec::new(),
        body: root.nodes.iter()
            .filter_map(|node| match node {
                &ast::Node::GlobalVariableDecl(ref variable_decl) =>
                    match &variable_decl.init_expr {
                        &Some(ref init_expr) =>
                            Some(ast::Statement::Assignment(ast::Assignment {
                                l_value: ast::LValue::VariableRef(ast::VariableRef {
                                    name: variable_decl.name.clone(),
                                    type_specifier: variable_decl.type_specifier.clone()
                                }),
                                expr: init_expr.clone()
                            })),
                        _ => None
                    },
                &ast::Node::ConstDecl(ref const_decl) =>
                    Some(ast::Statement::Assignment(ast::Assignment {
                        l_value: ast::LValue::VariableRef(ast::VariableRef {
                            name: const_decl.name.clone(),
                            type_specifier: const_decl.type_specifier.clone()
                        }),
                        expr: const_decl.init_expr.clone()
                    })),
                &ast::Node::Statement(ref statement) => Some(statement.clone()),
                &ast::Node::End => Some(ast::Statement::End),
                _ => None
            })
            .collect::<Vec<_>>()
    }
}

fn compile_function(
    function_decl: &ast::FunctionDecl,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    data_labels_map: &HashMap<String, usize>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::Function {

    let signature = il::FunctionSignature {
        name: function_decl.name.clone(),
        args: function_decl.args.iter().map(compile_single_variable).collect::<Vec<_>>()
    };

    let locals = compile_locals(function_decl, &signature, globals_index_map);

    let body = {
        let tables = Tables {
            un_op_index_map: un_op_index_map,
            un_op_impls_table: un_op_impls_table,
            bin_op_index_map: bin_op_index_map,
            bin_op_impls_table: bin_op_impls_table,
            globals: globals,
            globals_index_map: globals_index_map,
            locals: &locals,
            data_labels_map: data_labels_map,
            function_index_map: function_index_map,
            function_types_table: function_types_table
        };

        function_decl.body.iter()
            .map(|statement| compile_statement(statement, &tables))
            .collect::<Vec<_>>()
    };

    let stack_frame_size = locals.len();

    il::Function {
        signature: signature,
        locals: locals,
        stack_frame_size: stack_frame_size,
        body: body
    }
}

struct Tables<'a> {
    un_op_index_map: &'a HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &'a Vec<impls::FunctionImpl>,
    bin_op_index_map: &'a HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &'a Vec<impls::FunctionImpl>,
    globals: &'a Vec<il::Variable>,
    globals_index_map: &'a HashMap<String, usize>,
    locals: &'a Vec<il::Variable>,
    data_labels_map: &'a HashMap<String, usize>,
    function_index_map: &'a HashMap<String, usize>,
    function_types_table: &'a Vec<ValueType>
}

fn compile_locals(
    function_decl: &ast::FunctionDecl,
    signature: &il::FunctionSignature,
    globals_index_map: &HashMap<String, usize>) -> Vec<il::Variable> {

    let mut ret = Vec::new();

    for arg in signature.args.iter() {
        ret.push(il::Variable::SingleVariable(arg.clone()));
    }

    for statement in function_decl.body.iter() {
        compile_locals_visit_statement(statement, globals_index_map, &mut ret);
    }

    ret
}

fn compile_locals_visit_statement(
    statement: &ast::Statement,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) {

    match statement {
        &ast::Statement::ArrayDecl(_) |
        &ast::Statement::Restore(_) |
        &ast::Statement::Read(_) |
        &ast::Statement::FunctionCall(_) |
        &ast::Statement::End => (),

        &ast::Statement::If(ref if_statement) =>
            compile_locals_visit_if_statement(if_statement, globals_index_map, locals),
        &ast::Statement::While(ref while_statement) =>
            compile_locals_visit_while_statement(while_statement, globals_index_map, locals),
        &ast::Statement::For(ref for_statement) =>
            compile_locals_visit_for_statement(for_statement, globals_index_map, locals),
        &ast::Statement::Assignment(ref assignment) =>
            compile_locals_visit_assignment(assignment, globals_index_map, locals),

        _ => panic!("Unrecognized AST statement: {:#?}", statement)
    }
}

fn compile_locals_visit_if_statement(
    if_statement: &ast::If,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) {

    for statement in if_statement.body.iter() {
        compile_locals_visit_statement(statement, globals_index_map, locals);
    }

    if let Some(ref else_clause) = if_statement.else_clause {
        for statement in else_clause.body.iter() {
            compile_locals_visit_statement(statement, globals_index_map, locals);
        }
    }
}

fn compile_locals_visit_while_statement(
    while_statement: &ast::While,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) {

    for statement in while_statement.body.iter() {
        compile_locals_visit_statement(statement, globals_index_map, locals);
    }
}

fn compile_locals_visit_for_statement(
    for_statement: &ast::For,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) {

    compile_locals_visit_assignment(&for_statement.initialization, globals_index_map, locals);

    for statement in for_statement.body.iter() {
        compile_locals_visit_statement(statement, globals_index_map, locals);
    }
}

fn compile_locals_visit_assignment(
    assignment: &ast::Assignment,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) {

    let (l_value_name, value_type) = match &assignment.l_value {
        &ast::LValue::VariableRef(ref variable_ref) =>
            (variable_ref.name.clone(), compile_value_type(&variable_ref.type_specifier)),
        &ast::LValue::ArrayElemRef(ref array_elem_ref) =>
            (array_elem_ref.array_name.clone(), compile_value_type(&array_elem_ref.type_specifier))
    };

    if !locals.iter().any(|x| &l_value_name == x.name()) && !globals_index_map.contains_key(&l_value_name) {
        locals.push(match &assignment.l_value {
            &ast::LValue::VariableRef(_) => il::Variable::SingleVariable(il::SingleVariable {
                name: l_value_name,
                is_const: false,
                value_type: value_type
            }),
            _ => panic!("Attempted assignment to undefined array element: {}", l_value_name)
        });
    }
}

fn compile_statement(statement: &ast::Statement, tables: &Tables) -> il::Statement {
    match statement {
        &ast::Statement::ArrayDecl(ref array_decl) => il::Statement::ArrayAlloc(compile_array_alloc(array_decl, tables)),
        &ast::Statement::If(ref if_statement) => il::Statement::If(compile_if_statement(if_statement, tables)),
        &ast::Statement::While(ref while_statement) => il::Statement::While(compile_while_statement(while_statement, tables)),
        &ast::Statement::For(ref for_statement) => il::Statement::For(compile_for_statement(for_statement, tables)),
        &ast::Statement::Restore(ref label_name) => il::Statement::Restore(*tables.data_labels_map.get(label_name).unwrap()),
        &ast::Statement::Read(ref l_value) => il::Statement::Read(compile_l_value(l_value, tables)),
        &ast::Statement::Assignment(ref assignment) => il::Statement::Assignment(compile_assignment(assignment, tables)),
        &ast::Statement::FunctionCall(ref function_call) => il::Statement::FunctionCall(compile_function_call(function_call, tables)),
        &ast::Statement::End => il::Statement::End,
        _ => panic!("Unrecognized AST statement: {:#?}", statement)
    }
}

fn compile_array_alloc(array_decl: &ast::ArrayDecl, tables: &Tables) -> il::ArrayAlloc {
    // TODO: Resolve local arrays
    // TODO: Try to reuse code between resolve_variable_ref
    match tables.globals_index_map.get(&array_decl.name) {
        Some(index) => il::ArrayAlloc {
            array_ref: il::ArrayRef::Global(*index),
            dimensions: array_decl.dimensions.iter()
                .map(|x| compile_expr(x, tables))
                .collect::<Vec<_>>(),
            value_type: compile_value_type(&array_decl.type_specifier)
        },
        _ => panic!("Unable to resolve array name: {:?}", array_decl)
    }
}

fn compile_if_statement(if_statement: &ast::If, tables: &Tables) -> il::If {
    il::If {
        condition: compile_expr(&if_statement.condition, tables),
        body: if_statement.body.iter()
            .map(|statement| compile_statement(statement, tables))
            .collect::<Vec<_>>(),
        else_clause: if_statement.else_clause.clone().map(
            |else_clause| else_clause.body.iter()
                .map(|statement| compile_statement(statement, tables))
                .collect::<Vec<_>>())
    }
}

fn compile_while_statement(while_statement: &ast::While, tables: &Tables) -> il::While {
    il::While {
        condition: compile_expr(&while_statement.condition, tables),
        body: while_statement.body.iter()
            .map(|statement| compile_statement(statement, tables))
            .collect::<Vec<_>>(),
    }
}

fn compile_for_statement(for_statement: &ast::For, tables: &Tables) -> il::For {
    let index_l_value = &for_statement.initialization.l_value;
    let index_variable_ref = match index_l_value {
        &ast::LValue::VariableRef(ref variable_ref) => variable_ref.clone(),
        _ => panic!("Array element ref used as for loop iterator: {:#?}", index_l_value)
    };
    il::For {
        initialization: compile_assignment(&for_statement.initialization, tables),
        condition: compile_expr(
            &Box::new(ast::Expr::BinOp(ast::BinOp {
                op: ast::Op::LtEq,
                lhs: Box::new(ast::Expr::VariableRef(ast::VariableRef {
                    name: index_variable_ref.name.clone(),
                    type_specifier: None
                })),
                rhs: for_statement.to.clone()
            })), tables),
        increment: compile_assignment(
            &ast::Assignment {
                l_value: index_l_value.clone(),
                expr: Box::new(ast::Expr::BinOp(ast::BinOp {
                    op: ast::Op::Add,
                    lhs: Box::new(ast::Expr::VariableRef(index_variable_ref.clone())),
                    rhs: for_statement.step.clone().unwrap_or(Box::new(ast::Expr::IntegerLiteral(1)))
                }))
            }, tables),
        body: for_statement.body.iter()
            .map(|statement| compile_statement(statement, tables))
            .collect::<Vec<_>>()
    }
}

fn compile_assignment(assignment: &ast::Assignment, tables: &Tables) -> il::Assignment {
    let l_value = compile_l_value(&assignment.l_value, tables);

    let expr = compile_expr(&assignment.expr, tables);
    let value_type = l_value.value_type();

    il::Assignment {
        l_value: l_value,
        expr: insert_cast(expr, value_type)
    }
}

fn insert_cast(expr: Box<il::Expr>, value_type: ValueType) -> Box<il::Expr> {
    if expr.value_type() == value_type {
        expr
    } else {
        Box::new(il::Expr::Cast(il::Cast {
            expr: expr,
            target_type: value_type
        }))
    }
}

fn compile_function_call(function_call: &ast::FunctionCallOrArrayElemRef, tables: &Tables) -> il::FunctionCall {
    let function_index = match tables.function_index_map.get(&function_call.function_name) {
        Some(index) => *index,
        _ => panic!("Unresolved function: {}", function_call.function_name)
    };

    il::FunctionCall {
        function_index: function_index,
        arguments: function_call.arguments.iter()
            // TODO: Insert casts for argument types
            .map(|expr| compile_expr(expr, tables))
            .collect::<Vec<_>>(),
        return_type: tables.function_types_table[function_index]
    }
}

fn compile_l_value(l_value: &ast::LValue, tables: &Tables) -> il::LValue {
    match l_value {
        &ast::LValue::VariableRef(ref variable_ref) => il::LValue::VariableRef(resolve_variable_ref(&variable_ref.name, tables)),
        &ast::LValue::ArrayElemRef(ref array_elem_ref) => il::LValue::ArrayElemRef(compile_array_elem_ref(array_elem_ref, tables))
    }
}

fn resolve_variable_ref(name: &String, tables: &Tables) -> il::VariableRef {
    for (index, local) in tables.locals.iter().enumerate() {
        if name == local.name() {
            return il::VariableRef::Local(il::LocalVariableRef {
                local_index: index,
                value_type: local.value_type()
            });
        }
    }

    if let Some(index) = tables.globals_index_map.get(name) {
        return il::VariableRef::Global(il::GlobalVariableRef {
            global_index: *index,
            value_type: tables.globals[*index].value_type()
        });
    }

    panic!("Unable to resolve variable ref: {}", name);
}

fn compile_array_elem_ref(array_elem_ref: &ast::ArrayElemRef, tables: &Tables) -> il::ArrayElemRef {
    let dimensions =
        array_elem_ref.dimensions.iter()
        .map(|x| compile_expr(x, tables))
        .collect::<Vec<_>>();

    // TODO: Resolve local arrays
    // TODO: Try to reuse code between resolve_variable_ref
    if let Some(index) = tables.globals_index_map.get(&array_elem_ref.array_name) {
        return il::ArrayElemRef::Global(il::GlobalArrayElemRef {
            global_index: *index,
            dimensions: dimensions,
            value_type: tables.globals[*index].value_type()
        });
    }

    panic!("Unable to resolve array elem ref: {:?}", array_elem_ref);
}

fn compile_expr(expr: &ast::Expr, tables: &Tables) -> Box<il::Expr> {
    match expr {
        &ast::Expr::FloatLiteral(value) => Box::new(il::Expr::Float(value)),
        &ast::Expr::IntegerLiteral(value) => Box::new(il::Expr::Integer(value)),
        &ast::Expr::BoolLiteral(value) => Box::new(il::Expr::Bool(value)),
        &ast::Expr::StringLiteral(ref value) => Box::new(il::Expr::String(value.clone())),
        &ast::Expr::FunctionCallOrArrayElemRef(ref function_call_or_array_elem_ref) =>
            compile_function_call_or_array_elem_ref(function_call_or_array_elem_ref, tables),
        &ast::Expr::VariableRef(ref variable_ref) => compile_variable_ref(variable_ref, tables),
        &ast::Expr::UnOp(ref un_op) => Box::new(il::Expr::UnOp(compile_un_op(un_op, tables))),
        &ast::Expr::BinOp(ref bin_op) => Box::new(il::Expr::BinOp(compile_bin_op(bin_op, tables)))
    }
}

fn compile_function_call_or_array_elem_ref(
    function_call_or_array_elem_ref: &ast::FunctionCallOrArrayElemRef,
    tables: &Tables) -> Box<il::Expr> {

    let arguments_or_dimensions =
        function_call_or_array_elem_ref.arguments.iter()
        .map(|x| compile_expr(x, tables))
        .collect::<Vec<_>>();

    Box::new(match tables.function_index_map.get(&function_call_or_array_elem_ref.function_name) {
        Some(index) => il::Expr::FunctionCall(il::FunctionCall {
            function_index: *index,
            arguments: arguments_or_dimensions,
            return_type: tables.function_types_table[*index]
        }),
        _ => {
            // TODO: Resolve local arrays
            // TODO: Try to reuse code between resolve_variable_ref
            match tables.globals_index_map.get(&function_call_or_array_elem_ref.function_name) {
                Some(index) => il::Expr::ArrayElemRef(il::ArrayElemRef::Global(il::GlobalArrayElemRef {
                    global_index: *index,
                    dimensions: arguments_or_dimensions,
                    value_type: tables.globals[*index].value_type()
                })),
                _ => panic!("Unable to resolve function call or array elem ref: {:#?}", function_call_or_array_elem_ref)
            }
        }
    })
}

fn compile_variable_ref(variable_ref: &ast::VariableRef, tables: &Tables) -> Box<il::Expr> {
    Box::new(il::Expr::VariableRef(resolve_variable_ref(&variable_ref.name, tables)))
}

fn compile_un_op(un_op: &ast::UnOp, tables: &Tables) -> il::UnOp {
    let expr = compile_expr(&un_op.expr, tables);
    let expr_type = expr.value_type();

    let key = (un_op.op, expr_type);
    if let Some(impl_index) = tables.un_op_index_map.get(&key) {
        il::UnOp {
            impl_index: *impl_index,
            expr: expr,
            return_type: tables.un_op_impls_table[*impl_index].return_type
        }
    } else {
        panic!("Unrecognized or unsupported un op for key: {:?}", key);
    }
}

fn compile_bin_op(bin_op: &ast::BinOp, tables: &Tables) -> il::BinOp {
    let lhs = compile_expr(&bin_op.lhs, tables);
    let rhs = compile_expr(&bin_op.rhs, tables);
    let lhs_type = lhs.value_type();
    let rhs_type = rhs.value_type();

    let key = (bin_op.op, lhs_type, rhs_type);
    if let Some(impl_index) = tables.bin_op_index_map.get(&key) {
        il::BinOp {
            impl_index: *impl_index,
            lhs: lhs,
            rhs: rhs,
            return_type: tables.bin_op_impls_table[*impl_index].return_type
        }
    } else {
        panic!("Unrecognized or unsupported bin op for key: {:?}", key);
    }
}
