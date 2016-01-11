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
               function_call |
               comment),
           opt!(multispace)));

//named!(statement<ast::Node>, function_call);

named!(function_call<ast::Node>,
       chain!(
           function_name: identifier ~
               arguments: arguments,
           || ast::Node::Statement(
               ast::Statement::FunctionCall(
                   ast::FunctionCall {
                       function_name: function_name,
                       arguments: arguments
                   }))));

named!(identifier<String>,
       map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));

named!(arguments<ast::ArgumentList>,
       many0!(
           delimited!(
               opt!(multispace),
               string_literal,
               opt!(multispace))));

//named!(expression<ast::Expr>, string_literal);

named!(string_literal<ast::Expr>,
       chain!(
           contents: map_res!(
               map_res!(
                   delimited!(
                       char!('"'),
                       is_not!("\"\r\n"),
                       char!('"')),
                   str::from_utf8),
               FromStr::from_str),
           || ast::Expr::String(contents)));

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
