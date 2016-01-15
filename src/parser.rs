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
// a quick initializer for those yet (see
// https://github.com/rust-lang/rfcs/issues/542 for more
// info).
const KEYWORDS: [&'static str; 18] = [
    "Include",

    "Global",

    "End",

    "If",
    "Else",
    "EndIf",
    "Then",

    "While",
    "Wend",

    "For",
    "Next",
    "Step",

    "Select",
    "Case",

    "True",
    "False",

    "And",
    "Not"];

named!(root<ast::Root>,
       chain!(
           nodes: many0!(node),
           || ast::Root { nodes: nodes }));

named!(node<ast::Node>,
       delimited!(
           opt!(whitespace),
           alt!(
               include |
               global_decl |
               chain!(
                   statement: statement,
                   || ast::Node::Statement(statement))),
           opt!(whitespace)));

// Some more 1337 h4xx0rzzzz :P
named!(whitespace<&[u8]>,
       recognize!(many1!(alt!(multispace | comment))));

named!(comment<&[u8]>,
       recognize!(
           delimited!(
               char!(';'),
               is_not!(";\r\n"),
               alt!(tag!("\n") | tag!("\r\n")))));

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
                       expr)),
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

// TODO: Complete based off of https://github.com/Leushenko/Taranis/blob/master/BlitzBasic%20grammar.txt
named!(expr<BoxedExpr>,
       chain!(
           ops: many0!(
               chain!(
                   delimited!(
                       opt!(space),
                       tag!("Not"),
                       opt!(space)),
                   || ast::Op::Not)) ~
               expr: bitwise_expr,
           || reduce_un_op_expr(&ops, &expr, 0)));

named!(bitwise_expr<BoxedExpr>,
       chain!(
           lhs: comp_expr ~
               rhss: many0!(
                   pair!(bitwise_op, comp_expr)),
           || reduce_bin_op_expr(&lhs, &rhss, 0)));

named!(comp_expr<BoxedExpr>,
       chain!(
           lhs: sum_expr ~
               rhss: many0!(
                   pair!(comp_op, sum_expr)),
           || reduce_bin_op_expr(&lhs, &rhss, 0)));

named!(sum_expr<BoxedExpr>,
       chain!(
           lhs: shift_expr ~
               rhss: many0!(
                   pair!(sum_op, shift_expr)),
           || reduce_bin_op_expr(&lhs, &rhss, 0)));

named!(shift_expr<BoxedExpr>,
       chain!(
           lhs: mul_expr ~
               rhss: many0!(
                   pair!(shift_op, mul_expr)),
           || reduce_bin_op_expr(&lhs, &rhss, 0)));

named!(mul_expr<BoxedExpr>,
       chain!(
           lhs: unary_expr ~
               rhss: many0!(
                   pair!(mul_op, unary_expr)),
           || reduce_bin_op_expr(&lhs, &rhss, 0)));

named!(unary_expr<BoxedExpr>,
       chain!(
           ops: many0!(unary_op) ~
               expr: atomic_value,
           || reduce_un_op_expr(&ops, &expr, 0)));

named!(atomic_value<BoxedExpr>,
       delimited!(
           opt!(space),
           chain!(
               ret: alt!(
                   chain!(
                       float_literal: float_literal,
                       || ast::Expr::FloatLiteral(float_literal)) |
                   chain!(
                       integer_literal: integer_literal,
                       || ast::Expr::IntegerLiteral(integer_literal)) |
                   chain!(
                       bool_literal: bool_literal,
                       || ast::Expr::BoolLiteral(bool_literal)) |
                   chain!(
                       string_literal: string_literal,
                       || ast::Expr::StringLiteral(string_literal)) |
                   chain!(
                       function_call_expr: function_call_expr,
                       || ast::Expr::FunctionCall(
                           function_call_expr)) |
                   chain!(
                       variable_ref: variable_ref,
                       || ast::Expr::VariableRef(variable_ref))),
               || Box::new(ret)),
           opt!(space)));

named!(comp_op<ast::Op>,
       delimited!(
           opt!(space),
           alt!(
               chain!(tag!("="), || ast::Op::Eq) |
               chain!(tag!("<"), || ast::Op::Lt) |
               chain!(tag!(">"), || ast::Op::Gt)),
           opt!(space)));

named!(sum_op<ast::Op>,
       delimited!(
           opt!(space),
           alt!(
               chain!(tag!("+"), || ast::Op::Add) |
               chain!(tag!("-"), || ast::Op::Sub)),
           opt!(space)));

named!(shift_op<ast::Op>,
       delimited!(
           opt!(space),
           alt!(
               chain!(tag!("Shl"), || ast::Op::Shl) |
               chain!(tag!("Shr"), || ast::Op::Shr) |
               chain!(tag!("Sar"), || ast::Op::Sar)),
           opt!(space)));

named!(mul_op<ast::Op>,
       delimited!(
           opt!(space),
           alt!(
               chain!(tag!("*"), || ast::Op::Mul) |
               chain!(tag!("/"), || ast::Op::Div)),
           opt!(space)));

named!(bitwise_op<ast::Op>,
       delimited!(
           opt!(space),
           alt!(
               chain!(tag!("And"), || ast::Op::And) |
               chain!(tag!("Or"), || ast::Op::Or) |
               chain!(tag!("Xor"), || ast::Op::Xor)),
           opt!(space)));

named!(unary_op<ast::Op>,
       delimited!(
           opt!(space),
           chain!(tag!("-"), || ast::Op::Neg),
           opt!(space)));

// TODO: Better name?
fn reduce_un_op_expr(
    ops: &Vec<ast::Op>,
    expr: &BoxedExpr,
    index: usize) -> BoxedExpr {
    if index == ops.len() {
        expr.clone()
    } else {
        Box::new(ast::Expr::UnOp(ast::UnOp {
            op: ops[index].clone(),
            expr: reduce_un_op_expr(ops, expr, index + 1)
        }))
    }
}

// TODO: Better name?
fn reduce_bin_op_expr(
    lhs: &BoxedExpr,
    rhss: &Vec<(ast::Op, BoxedExpr)>,
    index: usize) -> BoxedExpr {
    if index == rhss.len() {
        lhs.clone()
    } else {
        let (ref op, ref rhs) = rhss[index];
        Box::new(ast::Expr::BinOp(ast::BinOp {
            op: op.clone(),
            lhs: lhs.clone(),
            rhs: reduce_bin_op_expr(&rhs, rhss, index + 1)
        }))
    }
}

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

named!(integer_literal<i32>,
       map_res!(
           map_res!(
               recognize!(preceded!(opt!(tag!("-")), digit)),
               str::from_utf8),
           FromStr::from_str));

named!(bool_literal<bool>,
       alt!(
           chain!(tag!("True"), || true) |
           chain!(tag!("False"), || false)));

named!(function_call_expr<ast::FunctionCall>,
       chain!(
           function_name: identifier ~
               type_specifier: opt!(type_specifier) ~
               arguments: delimited!(
                   tag!("("),
                   separated_list!(tag!(","), expr),
                   tag!(")")),
           || ast::FunctionCall {
               function_name: function_name,
               type_specifier: type_specifier,
               arguments: arguments
           }));

named!(variable_ref<ast::VariableRef>,
       chain!(
           name: identifier ~
               type_specifier: opt!(type_specifier),
           || ast::VariableRef {
               name: name,
               type_specifier: type_specifier
           }));

named!(statement<ast::Statement>,
       alt!(
           chain!(
               if_statement: if_statement,
               || ast::Statement::If(if_statement)) |
           chain!(
               while_statement: while_statement,
               || ast::Statement::While(while_statement)) |
           chain!(
               for_statement: for_statement,
               || ast::Statement::For(for_statement)) |
           chain!(
               select: select,
               || ast::Statement::Select(select)) |
           chain!(
               variable_assignment: variable_assignment,
               || ast::Statement::VariableAssignment(
                   variable_assignment)) |
           chain!(
               function_call: function_call_statement,
               || ast::Statement::FunctionCall(function_call))));

named!(if_statement<ast::If>,
       chain!(
           tag!("If") ~
               space ~
               ret: alt!(
                   chain!(
                       condition: expr ~
                       body: statement_list ~
                           opt!(whitespace) ~
                       else_clause: opt!(else_clause) ~
                           opt!(whitespace) ~
                           tag!("EndIf"),
                       || ast::If {
                           condition: condition,
                           body: body,
                           else_clause: else_clause
                       }) |
                   chain!(
                       condition: expr ~
                           tag!("Then") ~
                           space ~
                           body: separated_nonempty_list!(
                               preceded!(opt!(space), tag!(":")),
                               statement),
                       || ast::If {
                           condition: condition,
                           body: body,
                           else_clause: None
                       })),
       || ret));

named!(statement_list<ast::StatementList>,
       many0!(preceded!(opt!(whitespace), statement)));

named!(else_clause<ast::ElseClause>,
       chain!(
           tag!("Else") ~
               whitespace ~
               body: statement_list,
           || ast::ElseClause {
               body: body
           }));

named!(while_statement<ast::While>,
       chain!(
           tag!("While") ~
               space ~
               condition: expr ~
               body: statement_list ~
               opt!(whitespace) ~
               tag!("Wend"),
           || ast::While {
               condition: condition,
               body: body
           }));

named!(for_statement<ast::For>,
       chain!(
           tag!("For") ~
               space ~
               initialization: variable_assignment ~
               tag!("To") ~
               space ~
               // TODO: Probably too permissive
               to: atomic_value ~
               step: opt!(
                   chain!(
                       tag!("Step") ~
                           space ~
                           // TODO: Probably too permissive
                           value: atomic_value,
                       || value)) ~
               body: statement_list ~
               opt!(whitespace) ~
               tag!("Next"),
           || ast::For {
               initialization: initialization,
               to: to,
               step: step,
               body: body
           }));

named!(select<ast::Select>,
       chain!(
           tag!("Select") ~
               space ~
               expr: expr ~
               arms: many0!(preceded!(opt!(whitespace), case_arm)) ~
               opt!(whitespace) ~
               tag!("End") ~
               space ~
               tag!("Select"),
           || ast::Select {
               expr: expr,
               arms: arms
           }));

named!(case_arm<ast::CaseArm>,
       chain!(
           tag!("Case") ~
               space ~
               // TODO: Probably too permissive
               value: atomic_value ~
               body: statement_list,
           || ast::CaseArm {
               value: value,
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
                   expr),
           || ast::VariableAssignment {
               variable: ast::VariableRef {
                   name: variable_name,
                   type_specifier: type_specifier
               },
               expr: expr
           }));

named!(function_call_statement<ast::FunctionCall>,
       chain!(
           function_name: identifier ~
               type_specifier: opt!(type_specifier) ~
               arguments: opt!(
                   alt!(
                       delimited!(
                           tag!("("),
                           separated_list!(tag!(","), expr),
                           tag!(")")) |
                       separated_nonempty_list!(
                           tag!(","), expr))),
           || ast::FunctionCall {
               function_name: function_name,
               type_specifier: type_specifier,
               arguments: arguments.unwrap_or(Vec::new())
           }));
