use super::super::minifb::{Window, Scale};
use super::super::time;
use super::super::ast;
use super::context::*;

use std::f32::consts;

pub fn build_impls_table() -> Vec<(&'static str, FunctionImpl)> {
    vec![
        ("Float", Box::new(float_cast)),

        ("Sin", Box::new(sin)),
        ("Cos", Box::new(cos)),

        ("AppTitle", Box::new(app_title)),
        ("Graphics", Box::new(graphics)),

        ("SetBuffer", Box::new(set_buffer)),
        ("BackBuffer", Box::new(back_buffer)),

        ("LockBuffer", Box::new(lock_buffer)),
        ("UnlockBuffer", Box::new(unlock_buffer)),

        ("WritePixelFast", Box::new(write_pixel_fast)),

        ("HidePointer", Box::new(hide_pointer)),

        ("MilliSecs", Box::new(millisecs)),

        ("KeyDown", Box::new(key_down)),
        ("MouseDown", Box::new(mouse_down)),

        ("Cls", Box::new(cls)),
        ("Flip", Box::new(flip)),

        ("Color", Box::new(color)),
        ("Text", Box::new(text))]
}

fn float_cast(_: &mut Context, args: &Vec<Value>) -> Value {
    let arg = &args[0];
    Value::Float(match arg {
        &Value::Integer(x) => x as f32,
        &Value::Float(x) => x,
        _ => panic!("Unable to cast value to float: {:?}", arg)
    })
}

fn sin(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(degrees_to_radians(args[0].as_float()).sin())
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees / 180.0 * consts::PI
}

fn cos(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(degrees_to_radians(args[0].as_float()).cos())
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
            Scale::X1).unwrap());

    context.program_state.width = width;
    context.program_state.height = height;
    context.program_state.back_buffer = vec![0; (width * height) as usize];

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

fn lock_buffer(_: &mut Context, _: &Vec<Value>) -> Value {
    println!("LockBuffer called (and ignored)");

    Value::Unit
}

fn write_pixel_fast(context: &mut Context, args: &Vec<Value>) -> Value {
    let x = args[0].as_integer();
    let y = args[1].as_integer();
    let color = args[2].as_integer() as u32;

    context.program_state.back_buffer[(y * context.program_state.width + x) as usize] = color;

    Value::Unit
}

fn unlock_buffer(_: &mut Context, _: &Vec<Value>) -> Value {
    println!("UnlockBuffer called (and ignored)");

    Value::Unit
}

fn hide_pointer(_context: &mut Context, _args: &Vec<Value>) -> Value {
    println!("WARNING: HidePointer called but not yet implemented");

    Value::Integer(0)
}

fn millisecs(_: &mut Context, _: &Vec<Value>) -> Value {
    Value::Integer((time::precise_time_ns() / 1000000) as i32)
}

fn key_down(_context: &mut Context, _args: &Vec<Value>) -> Value {
    println!("WARNING: KeyDown called but not yet implemented");

    Value::Bool(false)
}

fn mouse_down(_context: &mut Context, _args: &Vec<Value>) -> Value {
    println!("WARNING: MouseDown called but not yet implemented");

    Value::Bool(false)
}

fn cls(_context: &mut Context, _args: &Vec<Value>) -> Value {
    println!("WARNING: Cls called but not yet implemented");

    Value::Unit
}

fn flip(context: &mut Context, _: &Vec<Value>) -> Value {
    println!("WARNING: Flip argument ignored");

    let buffer = &context.program_state.back_buffer;
    if let Some(ref mut window) = context.program_state.window {
        window.update(buffer);
    }

    Value::Unit
}

fn color(_: &mut Context, _: &Vec<Value>) -> Value {
    println!("Color called (and ignored)");

    Value::Unit
}

fn text(_: &mut Context, _: &Vec<Value>) -> Value {
    println!("Text called (and ignored)");

    Value::Unit
}

pub fn build_un_op_impls_table() -> Vec<((ast::Op, ValueType), FunctionImpl)> {
    vec![
        ((ast::Op::Not, ValueType::Bool), Box::new(un_op_not_bool)),

        ((ast::Op::Neg, ValueType::Float), Box::new(un_op_neg_float))]
}

fn un_op_not_bool(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(!args[0].as_bool())
}

fn un_op_neg_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(-args[0].as_float())
}

pub fn build_bin_op_impls_table() -> Vec<((ast::Op, ValueType, ValueType), FunctionImpl)> {
    vec![
        ((ast::Op::GtEq, ValueType::Integer, ValueType::Integer), Box::new(bin_op_gt_eq_int_int)),

        ((ast::Op::Lt, ValueType::Float, ValueType::Integer), Box::new(bin_op_lt_float_int)),

        ((ast::Op::Gt, ValueType::Integer, ValueType::Integer), Box::new(bin_op_gt_int_int)),
        ((ast::Op::Gt, ValueType::Float, ValueType::Integer), Box::new(bin_op_gt_float_int)),

        ((ast::Op::Eq, ValueType::Integer, ValueType::Integer), Box::new(bin_op_eq_int_int)),

        ((ast::Op::And, ValueType::Bool, ValueType::Bool), Box::new(bin_op_and_bool_bool)),

        ((ast::Op::Add, ValueType::Integer, ValueType::Integer), Box::new(bin_op_add_int_int)),
        ((ast::Op::Add, ValueType::Integer, ValueType::Float), Box::new(bin_op_add_int_float)),
        ((ast::Op::Add, ValueType::Float, ValueType::Integer), Box::new(bin_op_add_float_int)),
        ((ast::Op::Add, ValueType::Float, ValueType::Float), Box::new(bin_op_add_float_float)),

        ((ast::Op::Add, ValueType::Integer, ValueType::String), Box::new(bin_op_add_int_string)),

        ((ast::Op::Sub, ValueType::Integer, ValueType::Integer), Box::new(bin_op_sub_int_int)),
        ((ast::Op::Sub, ValueType::Float, ValueType::Integer), Box::new(bin_op_sub_float_int)),
        ((ast::Op::Sub, ValueType::Float, ValueType::Float), Box::new(bin_op_sub_float_float)),

        ((ast::Op::Mul, ValueType::Integer, ValueType::Integer), Box::new(bin_op_mul_int_int)),
        ((ast::Op::Mul, ValueType::Integer, ValueType::Float), Box::new(bin_op_mul_int_float)),
        ((ast::Op::Mul, ValueType::Float, ValueType::Integer), Box::new(bin_op_mul_float_int)),
        ((ast::Op::Mul, ValueType::Float, ValueType::Float), Box::new(bin_op_mul_float_float)),

        ((ast::Op::Div, ValueType::Integer, ValueType::Integer), Box::new(bin_op_div_int_int)),
        ((ast::Op::Div, ValueType::Float, ValueType::Float), Box::new(bin_op_div_float_float)),

        ((ast::Op::Shl, ValueType::Integer, ValueType::Integer), Box::new(bin_op_shl_int_int)),
        ((ast::Op::Shl, ValueType::Float, ValueType::Integer), Box::new(bin_op_shl_float_int)),

        ((ast::Op::Shr, ValueType::Integer, ValueType::Integer), Box::new(bin_op_shr_int_int)),
        ((ast::Op::Shr, ValueType::Float, ValueType::Integer), Box::new(bin_op_shr_float_int))]
}

fn bin_op_gt_eq_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_integer() >= args[1].as_integer())
}

fn bin_op_lt_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_float() < (args[1].as_integer() as f32))
}

fn bin_op_gt_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_integer() > args[1].as_integer())
}

fn bin_op_gt_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_float() > (args[1].as_integer() as f32))
}

fn bin_op_eq_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_integer() == args[1].as_integer())
}

fn bin_op_and_bool_bool(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_bool() && args[1].as_bool())
}

fn bin_op_add_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() + args[1].as_integer())
}

fn bin_op_add_int_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float((args[0].as_integer() as f32) + args[1].as_float())
}

fn bin_op_add_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float() + (args[1].as_integer() as f32))
}

fn bin_op_add_float_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float() + args[1].as_float())
}

fn bin_op_add_int_string(_: &mut Context, args: &Vec<Value>) -> Value {
    println!("WARNING: Int + String called; rhs was returned");

    args[1].clone()
}

fn bin_op_sub_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() - args[1].as_integer())
}

fn bin_op_sub_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float() - (args[1].as_integer() as f32))
}

fn bin_op_sub_float_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float() - args[1].as_float())
}

fn bin_op_mul_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() * args[1].as_integer())
}

fn bin_op_mul_int_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float((args[0].as_integer() as f32) * args[1].as_float())
}

fn bin_op_mul_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float() * (args[1].as_integer() as f32))
}

fn bin_op_mul_float_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float() * args[1].as_float())
}

fn bin_op_div_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() / args[1].as_integer())
}

fn bin_op_div_float_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float() / args[1].as_float())
}

fn bin_op_shl_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() << args[1].as_integer())
}

fn bin_op_shl_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer((args[0].as_float() as i32) << args[1].as_integer())
}

fn bin_op_shr_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(args[0].as_integer() >> args[1].as_integer())
}

fn bin_op_shr_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer((args[0].as_float() as i32) >> args[1].as_integer())
}
