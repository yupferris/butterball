use std::str;
use std::str::FromStr;

use super::ast;

use nom::{IResult, alpha, multispace};

pub fn parse(source: &String) -> Result<ast::Root, String> {
    match root(source.as_bytes()) {
        IResult::Done(_, root) => Ok(root),
        _ => unreachable!()
    }
}

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
               statement),
           opt!(multispace)));

named!(statement<ast::Node>,
       chain!(
           statement: function_call,
           || ast::Node::Statement(
               ast::Statement::FunctionCall(statement))));

named!(include<ast::Node>,
       chain!(
           tag!("Include") ~
               multispace ~
               expr: string_literal,
           || ast::Node::Include(expr)));

named!(global_decl<ast::Node>,
       chain!(
           tag!("Global") ~
               multispace ~
               name: identifier ~
               type_specifier: opt!(type_specifier) ~
               init_expr: opt!(
                   preceded!(
                       delimited!(
                           opt!(multispace),
                           tag!("="),
                           opt!(multispace)),
                       expression)),
           || ast::Node::GlobalDecl(ast::GlobalDecl {
               name: name,
               type_specifier: type_specifier,
               init_expr: init_expr
           })));

named!(type_specifier<ast::TypeSpecifier>,
       alt!(
           chain!(tag!("%"), || ast::TypeSpecifier::Int) |
           chain!(tag!("#"), || ast::TypeSpecifier::Float) |
           chain!(tag!("$"), || ast::TypeSpecifier::String)));

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

named!(identifier<String>,
       map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));

named!(argument_list<ast::ArgumentList>,
       delimited!(
           opt!(tag!("(")),
           separated_list!(tag!(","), expression),
           opt!(tag!(")"))));

named!(expression<ast::Expr>,
       delimited!(
           opt!(multispace),
           alt!(
               chain!(
                   string: string_literal,
                   || ast::Expr::String(string)) |
               chain!(
                   function_call: function_call,
                   || ast::Expr::FunctionCall(function_call))),
           opt!(multispace)));

named!(string_literal<String>,
       map_res!(
           map_res!(
               delimited!(
                   char!('"'),
                   is_not!("\"\r\n"),
                   char!('"')),
               str::from_utf8),
           FromStr::from_str));

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
