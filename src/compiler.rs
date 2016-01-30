use super::ast;
use super::il;

// TODO: Proper error handling?
pub fn compile(root: &ast::Root) -> il::Program {
    il::Program {
        function_table: compile_functions(root)
    }
}

fn compile_functions(root: &ast::Root) -> Vec<il::Function> {
    let mut ret =
        root.nodes.iter()
        .filter_map(|node| match node {
            &ast::Node::FunctionDecl(ref function_decl) => Some(il::Function { name: function_decl.name.clone() }),
            _ => None
        })
        .collect::<Vec<_>>();
    ret
}
