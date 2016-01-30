use super::value::*;
use super::ast;
use super::il;

use std::collections::HashMap;

// TODO: Proper error handling?
pub fn compile(root: &ast::Root) -> il::Program {
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
        .map(|x| compile_function(x, &globals_index_map, &function_index_map))
        .collect::<Vec<_>>();

    // We build a main function as an AST node so later we can compile it like any other function declaration.
    let main_function_ast = build_main_function_ast(root);
    //println!("Main function AST: {:#?}", main_function_ast);
    let main_function = compile_function(&main_function_ast, &globals_index_map, &function_index_map);

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
    globals_index_map: &HashMap<String, usize>,
    function_index_map: &HashMap<String, usize>) -> il::Function {

    let signature = il::FunctionSignature {
        name: function_decl.name.clone(),
        args: function_decl.args.iter().map(compile_single_variable).collect::<Vec<_>>()
    };

    println!("Compiling function: {}", signature.name);

    let locals = compile_locals(function_decl, &signature, globals_index_map);

    println!("Locals: {:#?}", locals);

    panic!("Dunno how to compile statements just yet :)")

    /*let body = function_decl.body.iter().map(compile_statement).collect::<Vec<_>>();

    il::Function {
        signature: signature,
        body: body
    }*/
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

fn compile_statement(statement: &ast::Statement) -> il::Statement {
    panic!("Unrecognized AST statement: {:#?}", statement)
}
