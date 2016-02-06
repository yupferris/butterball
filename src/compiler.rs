use super::value::*;
use super::ast;
use super::il;
use super::impls;

use std::collections::HashMap;

// TODO: Proper error handling?
pub fn compile(root: &ast::Root) -> il::Program {
    let un_op_impls_pairs = impls::build_un_op_impls_table();
    let un_op_impls_index_map =
        un_op_impls_pairs.iter()
        .enumerate()
        .map(|(i, x)| (x.0.clone(), i))
        .collect::<HashMap<_, _>>();
    let un_op_impls_table =
        un_op_impls_pairs.into_iter()
        .map(|x| x.1)
        .collect::<Vec<_>>();

    let bin_op_impls_pairs = impls::build_bin_op_impls_table();
    let bin_op_impls_index_map =
        bin_op_impls_pairs.iter()
        .enumerate()
        .map(|(i, x)| (x.0.clone(), i))
        .collect::<HashMap<_, _>>();
    let bin_op_impls_table =
        bin_op_impls_pairs.into_iter()
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
            &un_op_impls_index_map,
            &un_op_impls_table,
            &bin_op_impls_index_map,
            &bin_op_impls_table,
            &globals,
            &globals_index_map,
            &function_index_map,
            &function_types_table))
        .map(|x| il::FunctionTableEntry::Function(x))
        .chain(function_impls.into_iter().map(|x| il::FunctionTableEntry::FunctionImpl(x)))
        .collect::<Vec<_>>();

    // We build a main function as an AST node so later we can compile it like any other function declaration.
    let main_function_ast = build_main_function_ast(root);
    //println!("Main function AST: {:#?}", main_function_ast);
    let main_function =
        compile_function(
            &main_function_ast,
            &un_op_impls_index_map,
            &un_op_impls_table,
            &bin_op_impls_index_map,
            &bin_op_impls_table,
            &globals,
            &globals_index_map,
            &function_index_map,
            &function_types_table);

    il::Program {
        globals: globals,
        un_op_impls_table: un_op_impls_table,
        bin_op_impls_table: bin_op_impls_table,
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
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::Function {

    let signature = il::FunctionSignature {
        name: function_decl.name.clone(),
        args: function_decl.args.iter().map(compile_single_variable).collect::<Vec<_>>()
    };

    let locals = compile_locals(function_decl, &signature, globals_index_map);

    let body = function_decl.body.iter()
        .map(|statement| compile_statement(
            statement,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            &locals,
            function_index_map,
            function_types_table))
        .collect::<Vec<_>>();

    il::Function {
        signature: signature,
        body: body
    }
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
        &ast::Statement::ArrayDecl(_) | &ast::Statement::FunctionCall(_) | &ast::Statement::End => (),

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

fn compile_statement(
    statement: &ast::Statement,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::Statement {

    match statement {
        &ast::Statement::ArrayDecl(ref array_decl) =>
            il::Statement::ArrayAlloc(
                compile_array_alloc(
                    array_decl,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table)),
        &ast::Statement::If(ref if_statement) =>
            il::Statement::If(
                compile_if_statement(
                    if_statement,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table)),
        &ast::Statement::While(ref while_statement) =>
            il::Statement::While(
                compile_while_statement(
                    while_statement,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table)),
        &ast::Statement::For(ref for_statement) =>
            il::Statement::For(
                compile_for_statement(
                    for_statement,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table)),
        &ast::Statement::Assignment(ref assignment) =>
            il::Statement::Assignment(
                compile_assignment(
                    assignment,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table)),
        &ast::Statement::FunctionCall(ref function_call) =>
            il::Statement::FunctionCall(
                compile_function_call(
                    function_call,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table)),
        &ast::Statement::End => il::Statement::End,
        _ => panic!("Unrecognized AST statement: {:#?}", statement)
    }
}

fn compile_array_alloc(
    array_decl: &ast::ArrayDecl,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::ArrayAlloc {

    // TODO: Resolve local arrays
    // TODO: Try to reuse code between resolve_variable_ref
    match globals_index_map.get(&array_decl.name) {
        Some(index) => il::ArrayAlloc {
            array_ref: il::ArrayRef::Global(*index),
            dimensions: array_decl.dimensions.iter()
                .map(|x| compile_expr(
                    x,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table))
                .collect::<Vec<_>>(),
            value_type: compile_value_type(&array_decl.type_specifier)
        },
        _ => panic!("Unable to resolve array name: {:?}", array_decl)
    }
}

fn compile_if_statement(
    if_statement: &ast::If,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::If {

    il::If {
        condition: compile_expr(
            &if_statement.condition,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table),
        body: if_statement.body.iter()
            .map(|statement|
                 compile_statement(
                     statement,
                     un_op_index_map,
                     un_op_impls_table,
                     bin_op_index_map,
                     bin_op_impls_table,
                     globals,
                     globals_index_map,
                     locals,
                     function_index_map,
                     function_types_table))
            .collect::<Vec<_>>(),
        else_clause: if_statement.else_clause.clone().map(
            |else_clause| else_clause.body.iter()
                .map(|statement|
                     compile_statement(
                         statement,
                         un_op_index_map,
                         un_op_impls_table,
                         bin_op_index_map,
                         bin_op_impls_table,
                         globals,
                         globals_index_map,
                         locals,
                         function_index_map,
                         function_types_table))
                .collect::<Vec<_>>())
    }
}

fn compile_while_statement(
    while_statement: &ast::While,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::While {

    il::While {
        condition: compile_expr(
            &while_statement.condition,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table),
        body: while_statement.body.iter()
            .map(|statement|
                 compile_statement(
                     statement,
                     un_op_index_map,
                     un_op_impls_table,
                     bin_op_index_map,
                     bin_op_impls_table,
                     globals,
                     globals_index_map,
                     locals,
                     function_index_map,
                     function_types_table))
            .collect::<Vec<_>>(),
    }
}

fn compile_for_statement(
    for_statement: &ast::For,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::For {

    let index_l_value = &for_statement.initialization.l_value;
    let index_variable_ref = match index_l_value {
        &ast::LValue::VariableRef(ref variable_ref) => variable_ref.clone(),
        _ => panic!("Array element ref used as for loop iterator: {:#?}", index_l_value)
    };
    il::For {
        initialization: compile_assignment(
            &for_statement.initialization,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table),
        condition: compile_expr(
            &Box::new(ast::Expr::BinOp(ast::BinOp {
                op: ast::Op::Gt,
                lhs: Box::new(ast::Expr::VariableRef(ast::VariableRef {
                    name: index_variable_ref.name.clone(),
                    type_specifier: None
                })),
                rhs: for_statement.to.clone()
            })),
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table),
        increment: compile_assignment(
            &ast::Assignment {
                l_value: index_l_value.clone(),
                expr: Box::new(ast::Expr::BinOp(ast::BinOp {
                    op: ast::Op::Add,
                    lhs: Box::new(ast::Expr::VariableRef(index_variable_ref.clone())),
                    rhs: for_statement.step.clone().unwrap_or(Box::new(ast::Expr::IntegerLiteral(1)))
                }))
            },
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table),
        body: for_statement.body.iter()
            .map(|statement|
                 compile_statement(
                     statement,
                     un_op_index_map,
                     un_op_impls_table,
                     bin_op_index_map,
                     bin_op_impls_table,
                     globals,
                     globals_index_map,
                     locals,
                     function_index_map,
                     function_types_table))
            .collect::<Vec<_>>()
    }
}

fn compile_assignment(
    assignment: &ast::Assignment,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::Assignment {

    il::Assignment {
        l_value: compile_l_value(
            &assignment.l_value,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table),
        expr: compile_expr(
            &assignment.expr,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table)
    }
}

fn compile_function_call(
    function_call: &ast::FunctionCallOrArrayElemRef,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::FunctionCall {

    let function_index = match function_index_map.get(&function_call.function_name) {
        Some(index) => *index,
        _ => panic!("Unresolved function: {}", function_call.function_name)
    };

    il::FunctionCall {
        function_index: function_index,
        arguments: function_call.arguments.iter()
            .map(|expr| compile_expr(
                expr,
                un_op_index_map,
                un_op_impls_table,
                bin_op_index_map,
                bin_op_impls_table,
                globals,
                globals_index_map,
                locals,
                function_index_map,
                function_types_table))
            .collect::<Vec<_>>(),
        return_type: function_types_table[function_index]
    }
}

fn compile_l_value(
    l_value: &ast::LValue,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::LValue {

    match l_value {
        &ast::LValue::VariableRef(ref variable_ref) =>
            il::LValue::VariableRef(resolve_variable_ref(&variable_ref.name, globals_index_map, locals)),
        &ast::LValue::ArrayElemRef(ref array_elem_ref) =>
            il::LValue::ArrayElemRef(
                compile_array_elem_ref(
                    array_elem_ref,
                    un_op_index_map,
                    un_op_impls_table,
                    bin_op_index_map,
                    bin_op_impls_table,
                    globals,
                    globals_index_map,
                    locals,
                    function_index_map,
                    function_types_table))
    }
}

fn resolve_variable_ref(
    name: &String,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>) -> il::VariableRef {

    for (index, local) in locals.iter().enumerate() {
        if name == local.name() {
            return il::VariableRef::Local(index);
        }
    }

    if let Some(index) = globals_index_map.get(name) {
        return il::VariableRef::Global(*index);
    }

    panic!("Unable to resolve variable ref: {}", name);
}

fn compile_array_elem_ref(
    array_elem_ref: &ast::ArrayElemRef,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::ArrayElemRef {

    let dimensions =
        array_elem_ref.dimensions.iter()
        .map(|x| compile_expr(
            x,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table))
        .collect::<Vec<_>>();

    // TODO: Resolve local arrays
    // TODO: Try to reuse code between resolve_variable_ref
    if let Some(index) = globals_index_map.get(&array_elem_ref.array_name) {
        return il::ArrayElemRef::Global(il::GlobalArrayElemRef {
            global_index: *index,
            dimensions: dimensions,
            value_type: globals[*index].value_type()
        });
    }

    panic!("Unable to resolve array elem ref: {:?}", array_elem_ref);
}

fn compile_expr(
    expr: &Box<ast::Expr>,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> Box<il::Expr> {

    match **expr {
        ast::Expr::FloatLiteral(value) => Box::new(il::Expr::Float(value)),
        ast::Expr::IntegerLiteral(value) => Box::new(il::Expr::Integer(value)),
        ast::Expr::StringLiteral(ref value) => Box::new(il::Expr::String(value.clone())),
        ast::Expr::FunctionCallOrArrayElemRef(ref function_call_or_array_elem_ref) =>
            compile_function_call_or_array_elem_ref(
                function_call_or_array_elem_ref,
                un_op_index_map,
                un_op_impls_table,
                bin_op_index_map,
                bin_op_impls_table,
                globals,
                globals_index_map,
                locals,
                function_index_map,
                function_types_table),
        ast::Expr::VariableRef(ref variable_ref) => compile_variable_ref(variable_ref, globals_index_map, locals),
        ast::Expr::UnOp(ref un_op) =>
            Box::new(il::Expr::UnOp(compile_un_op(
                un_op,
                un_op_index_map,
                un_op_impls_table,
                bin_op_index_map,
                bin_op_impls_table,
                globals,
                globals_index_map,
                locals,
                function_index_map,
                function_types_table))),
        ast::Expr::BinOp(ref bin_op) =>
            Box::new(il::Expr::BinOp(compile_bin_op(
                bin_op,
                un_op_index_map,
                un_op_impls_table,
                bin_op_index_map,
                bin_op_impls_table,
                globals,
                globals_index_map,
                locals,
                function_index_map,
                function_types_table))),
        _ => panic!("Unrecognized AST expression: {:#?}", expr)
    }
}

fn compile_function_call_or_array_elem_ref(
    function_call_or_array_elem_ref: &ast::FunctionCallOrArrayElemRef,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> Box<il::Expr> {

    let arguments_or_dimensions =
        function_call_or_array_elem_ref.arguments.iter()
        .map(|x| compile_expr(
            x,
            un_op_index_map,
            un_op_impls_table,
            bin_op_index_map,
            bin_op_impls_table,
            globals,
            globals_index_map,
            locals,
            function_index_map,
            function_types_table))
        .collect::<Vec<_>>();

    Box::new(match function_index_map.get(&function_call_or_array_elem_ref.function_name) {
        Some(index) => il::Expr::FunctionCall(il::FunctionCall {
            function_index: *index,
            arguments: arguments_or_dimensions,
            return_type: function_types_table[*index]
        }),
        _ => {
            // TODO: Resolve local arrays
            // TODO: Try to reuse code between resolve_variable_ref
            match globals_index_map.get(&function_call_or_array_elem_ref.function_name) {
                Some(index) => il::Expr::ArrayElemRef(il::ArrayElemRef::Global(il::GlobalArrayElemRef {
                    global_index: *index,
                    dimensions: arguments_or_dimensions,
                    value_type: globals[*index].value_type()
                })),
                _ => panic!("Unable to resolve function call or array elem ref: {:?}", function_call_or_array_elem_ref)
            }
        }
    })
}

fn compile_variable_ref(
    variable_ref: &ast::VariableRef,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>) -> Box<il::Expr> {

    Box::new(il::Expr::VariableRef(resolve_variable_ref(&variable_ref.name, globals_index_map, locals)))
}

fn compile_un_op(
    un_op: &ast::UnOp,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::UnOp {

    let expr = compile_expr(
        &un_op.expr,
        un_op_index_map,
        un_op_impls_table,
        bin_op_index_map,
        bin_op_impls_table,
        globals,
        globals_index_map,
        locals,
        function_index_map,
        function_types_table);

    let expr_type = get_expr_type(&expr, globals, locals);

    let key = (un_op.op, expr_type);
    if let Some(impl_index) = un_op_index_map.get(&key) {
        il::UnOp {
            impl_index: *impl_index,
            expr: expr,
            return_type: un_op_impls_table[*impl_index].return_type
        }
    } else {
        panic!("Unrecognized or unsupported un op for key: {:?}", key);
    }
}

fn compile_bin_op(
    bin_op: &ast::BinOp,
    un_op_index_map: &HashMap<(ast::Op, ValueType), usize>,
    un_op_impls_table: &Vec<impls::FunctionImpl>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    bin_op_impls_table: &Vec<impls::FunctionImpl>,
    globals: &Vec<il::Variable>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>,
    function_index_map: &HashMap<String, usize>,
    function_types_table: &Vec<ValueType>) -> il::BinOp {

    let lhs = compile_expr(
        &bin_op.lhs,
        un_op_index_map,
        un_op_impls_table,
        bin_op_index_map,
        bin_op_impls_table,
        globals,
        globals_index_map,
        locals,
        function_index_map,
        function_types_table);
    let rhs = compile_expr(
        &bin_op.rhs,
        un_op_index_map,
        un_op_impls_table,
        bin_op_index_map,
        bin_op_impls_table,
        globals,
        globals_index_map,
        locals,
        function_index_map,
        function_types_table);

    let lhs_type = get_expr_type(&lhs, globals, locals);
    let rhs_type = get_expr_type(&rhs, globals, locals);

    let key = (bin_op.op, lhs_type, rhs_type);
    if let Some(impl_index) = bin_op_index_map.get(&key) {
        il::BinOp {
            impl_index: *impl_index,
            lhs: lhs,
            rhs: rhs,
            return_type: bin_op_impls_table[*impl_index].return_type
        }
    } else {
        panic!("Unrecognized or unsupported bin op for key: {:?}", key);
    }
}

fn get_expr_type(
    expr: &Box<il::Expr>,
    globals: &Vec<il::Variable>,
    locals: &Vec<il::Variable>) -> ValueType {
    match **expr {
        il::Expr::Float(_) => ValueType::Float,
        il::Expr::Integer(_) => ValueType::Integer,
        il::Expr::String(_) => ValueType::String,
        il::Expr::FunctionCall(ref function_call) => function_call.return_type,
        il::Expr::ArrayElemRef(ref array_elem_ref) => array_elem_ref.value_type(),
        il::Expr::VariableRef(ref variable_ref) => get_variable_ref_type(variable_ref, globals, locals),
        il::Expr::UnOp(ref un_op) => un_op.return_type,
        il::Expr::BinOp(ref bin_op) => bin_op.return_type
    }
}

fn get_variable_ref_type(
    variable_ref: &il::VariableRef,
    globals: &Vec<il::Variable>,
    locals: &Vec<il::Variable>) -> ValueType {

    match variable_ref {
        &il::VariableRef::Global(index) => globals[index].value_type(),
        &il::VariableRef::Local(index) => locals[index].value_type()
    }
}
