use value::Value;

use super::context::Context;

pub fn create_image(context: &mut Context, args: &[Value]) -> Value {
    let buffer_handle = context.buffers.alloc(args[0].as_integer(), args[1].as_integer());
    Value::Integer(context.images.alloc(buffer_handle) as i32)
}

pub fn free_image(context: &mut Context, args: &[Value]) -> Value {
    context.images.free(args[0].as_integer() as usize);

    Value::Unit
}

pub fn image_buffer(context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(context.images[args[0].as_integer() as usize].buffer_handle as i32)
}

pub fn mid_handle(context: &mut Context, args: &[Value]) -> Value {
    context.images[args[0].as_integer() as usize].mid_handle = true;

    Value::Unit
}
