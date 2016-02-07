use value::Value;

use super::context::Context;

pub fn app_title(context: &mut Context, args: &[Value]) -> Value {
    context.window.app_title(args[0].as_string());

    Value::Unit
}

pub fn graphics(context: &mut Context, args: &[Value]) -> Value {
    context.window.graphics(args[0].as_integer(), args[1].as_integer(), args[2].as_integer(), args[3].as_integer());

    Value::Unit
}

pub fn flip(context: &mut Context, _args: &[Value]) -> Value {
    context.window.flip();

    Value::Unit
}

pub fn back_buffer(_context: &mut Context, _args: &[Value]) -> Value {
    println!("WARNING: BackBuffer called but not yet implemented");

    Value::Integer(0)
}

pub fn hide_pointer(_context: &mut Context, _args: &[Value]) -> Value {
    println!("WARNING: HidePointer called but not yet implemented");

    Value::Integer(0)
}

pub fn key_down(context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(context.window.key_down(args[0].as_integer()))
}

pub fn mouse_down(context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(context.window.mouse_down(args[0].as_integer()))
}

pub fn mouse_x(context: &mut Context, _args: &[Value]) -> Value {
    Value::Integer(context.window.mouse_x())
}

pub fn mouse_y(context: &mut Context, _args: &[Value]) -> Value {
    Value::Integer(context.window.mouse_y())
}
