use super::super::minifb::{Window, Scale};
use super::super::time;
use super::super::ast;
use super::context::*;

pub fn build_impls_table() -> Vec<(&'static str, FunctionImpl)> {
    vec![
        ("AppTitle", Box::new(app_title)),
        ("Graphics", Box::new(graphics)),
        ("SetBuffer", Box::new(set_buffer)),
        ("BackBuffer", Box::new(back_buffer)),
        ("HidePointer", Box::new(hide_pointer)),
        ("MilliSecs", Box::new(millisecs))]
}

fn app_title(context: &mut Context, args: &Vec<Value>) -> Value {
    context.program_state.app_title = args[0].as_string();
    println!("New app title: \"{}\"", context.program_state.app_title);

    Value::Unit
}

fn graphics(context: &mut Context, args: &Vec<Value>) -> Value {
    let width = args[0].as_integer();
    let height = args[1].as_integer();
    let bits = args[2].as_integer();
    let window_mode = args[3].as_integer();
    println!(
        "Graphics called: {}, {}, {}, {} (ignoring bits and window mode)",
        width,
        height,
        bits,
        window_mode);

    context.program_state.window =
        Some(Window::new(
            &context.program_state.app_title,
            width as usize,
            height as usize,
            Scale::X2).unwrap());

    Value::Unit
}

fn set_buffer(_context: &mut Context, _args: &Vec<Value>) -> Value {
    println!("WARNING: SetBuffer called but not yet implemented");

    Value::Unit
}

fn back_buffer(_context: &mut Context, _args: &Vec<Value>) -> Value {
    println!("WARNING: BackBuffer called but not yet implemented");

    Value::Integer(0)
}

fn hide_pointer(_context: &mut Context, _args: &Vec<Value>) -> Value {
    println!("WARNING: HidePointer called but not yet implemented");

    Value::Integer(0)
}

fn millisecs(_: &mut Context, _: &Vec<Value>) -> Value {
    Value::Integer((time::precise_time_ns() / 1000000) as i32)
}

pub fn build_bin_op_impls_table() -> Vec<((ast::Op, ValueType, ValueType), FunctionImpl)> {
    vec![
        ((ast::Op::Eq, ValueType::Integer, ValueType::Integer), Box::new(bin_op_eq_int_int)),

        ((ast::Op::Mul, ValueType::Integer, ValueType::Integer), Box::new(bin_op_mul_int_int)),
        ((ast::Op::Mul, ValueType::Integer, ValueType::Float), Box::new(bin_op_mul_int_float)),

        ((ast::Op::Div, ValueType::Integer, ValueType::Integer), Box::new(bin_op_div_int_int))]
}

fn bin_op_eq_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_integer() == args[1].as_integer())
}

fn bin_op_mul_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() * args[1].as_integer())
}

fn bin_op_mul_int_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float((args[0].as_integer() as f32) * args[1].as_float())
}

fn bin_op_div_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() / args[1].as_integer())
}
