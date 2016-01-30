use super::value::*;
use super::ast;
use super::il;

// TODO: Proper error handling?
pub fn compile(root: &ast::Root) -> il::Program {
    let globals = compile_globals(root);
    let function_table = compile_functions(root);

    // We build a main function as an AST node so later we can compile it like any other function declaration.
    let main_function_ast = build_main_function_ast(root);
    println!("Main function: {:#?}", main_function_ast);

    il::Program {
        globals: globals,
        function_table: function_table
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
    il::Variable::SingleVariable(il::SingleVariable {
        name: variable_decl.name.clone(),
        is_const: false,
        value_type: compile_value_type(&variable_decl.type_specifier)
    })
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

fn compile_functions(root: &ast::Root) -> Vec<il::Function> {
    root.nodes.iter()
        .filter_map(|node| match node {
            &ast::Node::FunctionDecl(ref function_decl) => Some(il::Function { name: function_decl.name.clone() }),
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
