use std::str;
use std::str::FromStr;

use super::ast;

use nom::{IResult, alpha, newline};

pub fn parse(source: &String) -> Result<ast::Root, String> {
    match root_parser(source.as_bytes()) {
        IResult::Done(_, root) => Ok(root),
        _ => unreachable!()
    }
}

named!(root_parser<ast::Root>,
       chain!(
           nodes: many0!(comment_parser),
           || ast::Root { nodes: nodes }));

named!(comment_parser<ast::Node>,
       chain!(
           comment: map_res!(
               map_res!(
                   delimited!(
                       char!(';'),
                       is_not!(";\r\n"),
                       is_a!("\r\n")),
                   str::from_utf8),
               FromStr::from_str),
           || ast::Node::Comment(comment)));
