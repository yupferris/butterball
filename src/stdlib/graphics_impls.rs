use value::Value;

use super::context::Context;

pub fn cls(context: &mut Context, _args: &[Value]) -> Value {
    context.graphics.cls(&mut context.buffers);

    Value::Unit
}

pub fn color(_context: &mut Context, _args: &[Value]) -> Value {
    println!("Color called (and ignored)");

    Value::Unit
}

pub fn text(_context: &mut Context, args: &[Value]) -> Value {
    println!("Text called; drawing was ignored: {}, {}, {:?}", args[0].as_integer(), args[1].as_integer(), args[2].as_string());

    Value::Unit
}

pub fn draw_block(context: &mut Context, args: &[Value]) -> Value {
    context.graphics.draw_block(
        &context.images,
        &mut context.buffers,
        args[0].as_integer(),
        args[1].as_integer(),
        args[2].as_integer());

    Value::Unit
}

pub fn set_buffer(context: &mut Context, args: &[Value]) -> Value {
    context.graphics.set_buffer(args[0].as_integer());

    Value::Unit
}

pub fn lock_buffer(_context: &mut Context, _args: &[Value]) -> Value {
    println!("LockBuffer called (and ignored)");

    Value::Unit
}

pub fn unlock_buffer(_context: &mut Context, _args: &[Value]) -> Value {
    println!("UnlockBuffer called (and ignored)");

    Value::Unit
}

pub fn write_pixel_fast(context: &mut Context, args: &[Value]) -> Value {
    let buffer_handle = if args.len() == 4 { Some(args[3].as_integer()) } else { None };
    context.graphics.write_pixel_fast(
        &mut context.buffers,
        // TODO: Remove casts when impl's have proper argument types
        args[0].cast_to_integer().as_integer(),
        args[1].cast_to_integer().as_integer(),
        args[2].as_integer(),
        buffer_handle);

    Value::Unit
}
