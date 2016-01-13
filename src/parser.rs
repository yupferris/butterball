use std::str;
use std::str::FromStr;

use super::ast;

use nom::{IResult, alpha, alphanumeric, digit, space, multispace};

pub fn parse(source: &String) -> Result<ast::Root, String> {
    match root(source.as_bytes()) {
        IResult::Done(_, root) => Ok(root),
        _ => unreachable!()
    }
}

// TODO: Should really be a HashSet, but Rust doesn't have
// a quick initializer for those set (see
// https://github.com/rust-lang/rfcs/issues/542 for more
// info).
const KEYWORDS: [&'static str; 7] = [
    "Include",

    "Global",

    "If",
    "Else",
    "EndIf",

    "True",
    "False"];

named!(root<ast::Root>,
       chain!(
           nodes: many0!(node),
           || ast::Root { nodes: nodes }));

named!(node<ast::Node>,
       delimited!(
           opt!(multispace),
           alt!(
               comment |
               include |
               global_decl |
               chain!(
                   statement: statement,
                   || ast::Node::Statement(statement))),
           opt!(multispace)));

named!(comment<ast::Node>,
       chain!(
           comment: map_res!(
               map_res!(
                   delimited!(
                       char!(';'),
                       is_not!(";\r\n"),
                       alt!(tag!("\n") | tag!("\r\n"))),
                   str::from_utf8),
               FromStr::from_str),
           || ast::Node::Comment(comment)));

named!(include<ast::Node>,
       chain!(
           tag!("Include") ~
               space ~
               expr: string_literal,
           || ast::Node::Include(expr)));

named!(string_literal<String>,
       map_res!(
           map_res!(
               delimited!(
                   char!('"'),
                   is_not!("\"\r\n"),
                   char!('"')),
               str::from_utf8),
           FromStr::from_str));

named!(global_decl<ast::Node>,
       chain!(
           tag!("Global") ~
               space ~
               name: identifier ~
               type_specifier: opt!(type_specifier) ~
               init_expr: opt!(
                   preceded!(
                       delimited!(
                           opt!(space),
                           tag!("="),
                           opt!(space)),
                       expression)),
           || ast::Node::GlobalDecl(ast::GlobalDecl {
               name: name,
               type_specifier: type_specifier,
               init_expr: init_expr
           })));

named!(identifier<String>,
       map_res!(
           chain!(
               identifier_str: map_res!(
                   recognize!(
                       chain!(alpha ~ opt!(alphanumeric), || ())),
                   str::from_utf8) ~
                   _is_keyword: expr_opt!(
                       if KEYWORDS.contains(&identifier_str) {
                           None
                       } else {
                           Some(())
                       }),
               || identifier_str),
           FromStr::from_str));

named!(type_specifier<ast::TypeSpecifier>,
       alt!(
           chain!(tag!("%"), || ast::TypeSpecifier::Int) |
           chain!(tag!("#"), || ast::TypeSpecifier::Float) |
           chain!(tag!("$"), || ast::TypeSpecifier::String)));

type BoxedExpr = Box<ast::Expr>; // This was kindof a hack

named!(expression<BoxedExpr>,
       alt!(
           chain!(
               bin_op: complete!(bin_op),
               || Box::new(ast::Expr::BinOp(bin_op))) |
           complete!(term)));

named!(term<BoxedExpr>,
       delimited!(
           opt!(space),
           chain!(
               term: alt!(
                   chain!(
                       integer_literal: integer_literal,
                       || ast::Expr::IntegerLiteral(integer_literal)) |
                   chain!(
                       float_literal: float_literal,
                       || ast::Expr::FloatLiteral(float_literal)) |
                   chain!(
                       bool_literal: bool_literal,
                       || ast::Expr::BoolLiteral(bool_literal)) |
                   chain!(
                       string_literal: string_literal,
                       || ast::Expr::StringLiteral(string_literal)) |
                   chain!(
                       function_call: function_call,
                       || ast::Expr::FunctionCall(function_call)) |
                   chain!(
                       variable_ref: variable_ref,
                       || ast::Expr::VariableRef(variable_ref))),
               || Box::new(term)),
           opt!(space)));

named!(integer_literal<i32>,
       map_res!(
           map_res!(
               recognize!(preceded!(opt!(tag!("-")), digit)),
               str::from_utf8),
           FromStr::from_str));

named!(float_literal<f32>,
       map_res!(
           map_res!(
               recognize!(
                   alt!(
                       chain!(
                           integer_literal ~
                               tag!(".") ~
                               opt!(integer_literal),
                           || ()) |
                       chain!(
                           opt!(integer_literal) ~
                               tag!(".") ~
                               integer_literal,
                           || ()))),
               str::from_utf8),
           FromStr::from_str));

named!(bool_literal<bool>,
       alt!(
           chain!(tag!("True"), || true) |
           chain!(tag!("False"), || false)));

named!(function_call<ast::FunctionCall>,
       chain!(
           function_name: identifier ~
               type_specifier: opt!(type_specifier) ~
               arguments: argument_list,
           || ast::FunctionCall {
               function_name: function_name,
               type_specifier: type_specifier,
               arguments: arguments
           }));

named!(argument_list<ast::ArgumentList>,
       alt!(
           delimited!(
               tag!("("),
               separated_list!(tag!(","), expression),
               tag!(")")) |
           separated_nonempty_list!(tag!(","), expression)));

named!(variable_ref<ast::VariableRef>,
       chain!(
           name: identifier ~
               type_specifier: opt!(type_specifier),
           || ast::VariableRef {
               name: name,
               type_specifier: type_specifier
           }));

named!(bin_op<ast::BinOp>,
       chain!(
           lhs: term ~
               op: op ~
               rhs: term,
           || ast::BinOp {
               op: op,
               lhs: lhs,
               rhs: rhs
           }));

named!(op<ast::Op>,
       delimited!(
           opt!(space),
           alt!(
               chain!(tag!("="), || ast::Op::Equality) |
               chain!(tag!("+"), || ast::Op::Add) |
               chain!(tag!("*"), || ast::Op::Mul) |
               chain!(tag!("/"), || ast::Op::Div)),
           opt!(space)));

named!(statement<ast::Statement>,
       alt!(
           chain!(
               if_statement: if_statement,
               || ast::Statement::If(if_statement)) |
           chain!(
               statement: function_call,
               || ast::Statement::FunctionCall(statement)) |
           chain!(
               variable_assignment: variable_assignment,
               || ast::Statement::VariableAssignment(
                   variable_assignment))));

named!(if_statement<ast::If>,
       chain!(
           tag!("If") ~
               space ~
               condition: expression ~
               body: statement_list ~
               opt!(multispace) ~
           else_clause: opt!(else_clause) ~
               opt!(multispace) ~
               tag!("EndIf"),
           || ast::If {
               condition: condition,
               body: body,
               else_clause: else_clause
           }));

named!(statement_list<ast::StatementList>,
       many0!(preceded!(opt!(multispace), statement)));

named!(else_clause<ast::ElseClause>,
       chain!(
           tag!("Else") ~
               multispace ~
               body: statement_list,
           || ast::ElseClause {
               body: body
           }));

named!(variable_assignment<ast::VariableAssignment>,
       chain!(
           variable_name: identifier ~
               type_specifier: opt!(type_specifier) ~
               expr: preceded!(
                   delimited!(
                       opt!(space),
                       tag!("="),
                       opt!(space)),
                   expression),
           || ast::VariableAssignment {
               variable: ast::VariableRef {
                   name: variable_name,
                   type_specifier: type_specifier
               },
               expr: expr
           }));
