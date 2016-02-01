use super::value::*;
use super::ast;
use super::il;
use super::impls;

use std::collections::HashMap;

// TODO: Proper error handling?
pub fn compile(root: &ast::Root) -> il::Program {
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

    println!("Globals: {:#?}", globals);

    let function_asts = get_function_asts(root);
    let function_index_map =
        function_asts.iter()
        .enumerate()
        .map(|(i, x)| (x.name.clone(), i))
        .collect::<HashMap<_, _>>();

    let function_table =
        function_asts.iter()
        .map(|x| compile_function(x, &bin_op_impls_index_map, &globals_index_map, &function_index_map))
        .collect::<Vec<_>>();

    // We build a main function as an AST node so later we can compile it like any other function declaration.
    let main_function_ast = build_main_function_ast(root);
    //println!("Main function AST: {:#?}", main_function_ast);
    let main_function = compile_function(&main_function_ast, &bin_op_impls_index_map, &globals_index_map, &function_index_map);

    il::Program {
        globals: globals,
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
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    globals_index_map: &HashMap<String, usize>,
    function_index_map: &HashMap<String, usize>) -> il::Function {

    let signature = il::FunctionSignature {
        name: function_decl.name.clone(),
        args: function_decl.args.iter().map(compile_single_variable).collect::<Vec<_>>()
    };

    println!("Compiling function: {}", signature.name);

    let mut locals = compile_locals(function_decl, &signature, globals_index_map);

    println!("Locals: {:#?}", locals);

    let body = function_decl.body.iter()
        .map(|statement| compile_statement(statement, bin_op_index_map, globals_index_map, &mut locals)).collect::<Vec<_>>();

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
        &ast::Statement::For(ref for_statement) =>
            compile_locals_visit_for_statement(for_statement, globals_index_map, locals),
        &ast::Statement::Assignment(ref assignment) =>
            compile_locals_visit_assignment(assignment, globals_index_map, locals),
        _ => panic!("Unrecognized AST statement: {:#?}", statement)
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
            &ast::LValue::VariableRef(ref variable_ref) => il::Variable::SingleVariable(il::SingleVariable {
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
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) -> il::Statement {

    match statement {
        &ast::Statement::For(ref for_statement) =>
            il::Statement::For(compile_for_statement(for_statement, bin_op_index_map, globals_index_map, locals)),
        &ast::Statement::Assignment(ref assignment) =>
            il::Statement::Assignment(compile_assignment(assignment, bin_op_index_map, globals_index_map, locals)),
        _ => panic!("Unrecognized AST statement: {:#?}", statement)
    }
}

fn compile_for_statement(
    for_statement: &ast::For,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) -> il::For {

    let index_l_value = &for_statement.initialization.l_value;
    let index_variable_ref = match index_l_value {
        &ast::LValue::VariableRef(ref variable_ref) => variable_ref.clone(),
        _ => panic!("Array element ref used as for loop iterator: {:#?}", index_l_value)
    };
    il::For {
        initialization: compile_assignment(&for_statement.initialization, bin_op_index_map, globals_index_map, locals),
        condition: compile_expr(&Box::new(ast::Expr::BinOp(ast::BinOp {
            op: ast::Op::Gt,
            lhs: Box::new(ast::Expr::VariableRef(ast::VariableRef {
                name: index_variable_ref.name.clone(),
                type_specifier: None
            })),
            rhs: for_statement.to.clone()
        })), bin_op_index_map, globals_index_map, locals),
        increment: compile_assignment(&ast::Assignment {
            l_value: index_l_value.clone(),
            expr: Box::new(ast::Expr::BinOp(ast::BinOp {
                op: ast::Op::Add,
                lhs: Box::new(ast::Expr::VariableRef(index_variable_ref.clone())),
                rhs: for_statement.step.clone().unwrap_or(Box::new(ast::Expr::IntegerLiteral(1)))
            }))
        }, bin_op_index_map, globals_index_map, locals),
        body: for_statement.body.iter()
            .map(|statement| compile_statement(statement, bin_op_index_map, globals_index_map, locals)).collect::<Vec<_>>()
    }
}

fn compile_assignment(
    assignment: &ast::Assignment,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) -> il::Assignment {

    il::Assignment {
        l_value: compile_l_value(&assignment.l_value, globals_index_map, locals),
        expr: compile_expr(&assignment.expr, bin_op_index_map, globals_index_map, locals)
    }
}

fn compile_l_value(
    l_value: &ast::LValue,
    globals_index_map: &HashMap<String, usize>,
    locals: &mut Vec<il::Variable>) -> il::LValue {

    match l_value {
        &ast::LValue::VariableRef(ref variable_ref) =>
            il::LValue::VariableRef(resolve_variable_ref(&variable_ref.name, globals_index_map, locals)),
        _ => panic!("Not sure how to compile array elem ref l-values yet")
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

fn compile_expr(
    expr: &Box<ast::Expr>,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>) -> Box<il::Expr> {

    match **expr {
        ast::Expr::FloatLiteral(value) => Box::new(il::Expr::Float(value)),
        ast::Expr::IntegerLiteral(value) => Box::new(il::Expr::Integer(value)),
        ast::Expr::VariableRef(ref variable_ref) => compile_variable_ref(variable_ref, globals_index_map, locals),
        ast::Expr::BinOp(ref bin_op) => compile_bin_op(bin_op, bin_op_index_map, globals_index_map, locals),
        _ => panic!("Unrecognized AST expression: {:#?}", expr)
    }
}

fn compile_variable_ref(
    variable_ref: &ast::VariableRef,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>) -> Box<il::Expr> {

    Box::new(il::Expr::VariableRef(resolve_variable_ref(&variable_ref.name, globals_index_map, locals)))
}

fn compile_bin_op(
    bin_op: &ast::BinOp,
    bin_op_index_map: &HashMap<(ast::Op, ValueType, ValueType), usize>,
    globals_index_map: &HashMap<String, usize>,
    locals: &Vec<il::Variable>) -> Box<il::Expr> {

    let lhs = compile_expr(&bin_op.lhs, bin_op_index_map, globals_index_map, locals);
    let rhs = compile_expr(&bin_op.rhs, bin_op_index_map, globals_index_map, locals);

    let lhs_type = get_expr_type(&lhs, locals);
    let rhs_type = get_expr_type(&rhs, locals);

    let key = (bin_op.op.clone(), lhs_type, rhs_type);
    if let Some(impl_index) = bin_op_index_map.get(&key) {
        Box::new(il::Expr::BinOp(il::BinOp {
            impl_index: *impl_index,
            lhs: lhs,
            rhs: rhs
        }))
    } else {
        panic!("Unrecognized or unsupported bin op for key: {:?}", key);
    }
}

fn get_expr_type(
    expr: &Box<il::Expr>,
    locals: &Vec<il::Variable>) -> ValueType {
    match **expr {
        il::Expr::Float(_) => ValueType::Float,
        il::Expr::Integer(_) => ValueType::Integer,
        il::Expr::VariableRef(ref variable_ref) => get_variable_ref_type(variable_ref, locals),
        _ => panic!("Unrecognized IL expression: {:?}", expr)
    }
}

fn get_variable_ref_type(
    variable_ref: &il::VariableRef,
    locals: &Vec<il::Variable>) -> ValueType {

    match variable_ref {
        &il::VariableRef::Local(index) => locals[index].value_type(),
        _ => panic!("Unrecognized IL variable ref: {:?}", variable_ref)
    }
}
