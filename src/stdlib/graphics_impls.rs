use value::Value;

use super::context::Context;

pub fn cls(context: &mut Context, _: &[Value]) -> Value {
    context.graphics.cls(&mut context.window);

    Value::Unit
}

pub fn color(_: &mut Context, _: &[Value]) -> Value {
    println!("Color called (and ignored)");

    Value::Unit
}

pub fn text(_: &mut Context, args: &[Value]) -> Value {
    println!("Text called; drawing was ignored: {}, {}, {:?}", args[0].as_integer(), args[1].as_integer(), args[2].as_string());

    Value::Unit
}

pub fn set_buffer(_context: &mut Context, _args: &[Value]) -> Value {
    println!("WARNING: SetBuffer called but not yet implemented");

    Value::Unit
}

pub fn lock_buffer(_: &mut Context, _: &[Value]) -> Value {
    println!("LockBuffer called (and ignored)");

    Value::Unit
}

pub fn unlock_buffer(_: &mut Context, _: &[Value]) -> Value {
    println!("UnlockBuffer called (and ignored)");

    Value::Unit
}

pub fn write_pixel_fast(context: &mut Context, args: &[Value]) -> Value {
    context.graphics.write_pixel_fast(&mut context.window, args[0].as_integer(), args[1].as_integer(), args[2].as_integer());

    Value::Unit
}
