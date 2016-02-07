use value::Value;

use super::context::Context;

use std::f32::consts;

// TODO: Support type overloads
pub fn abs(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float().abs())
}

pub fn sin(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(degrees_to_radians(args[0].cast_to_float().as_float()).sin())
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees / 180.0 * consts::PI
}

pub fn cos(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(degrees_to_radians(args[0].cast_to_float().as_float()).cos())
}
