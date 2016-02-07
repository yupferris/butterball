use value::Value;

use super::context::Context;

pub fn seed_rnd(context: &mut Context, args: &[Value]) -> Value {
    context.rng.seed_rnd(args[0].as_integer());

    Value::Unit
}

pub fn rand(context: &mut Context, args: &[Value]) -> Value {
    let (low, high) = match args.len() {
        1 => (0, args[0].as_integer()),
        2 => (args[0].as_integer(), args[1].as_integer()),
        _ => panic!("Invalid number of arguments to Rand: {}", args.len())
    };
    Value::Integer(context.rng.rand(low, high))
}

pub fn rnd(context: &mut Context, args: &[Value]) -> Value {
    let (low, high) = match args.len() {
        1 => (0.0, args[0].as_float()),
        2 => (args[0].as_float(), args[1].as_float()),
        _ => panic!("Invalid number of arguments to Rnd: {}", args.len())
    };
    Value::Float(context.rng.rnd(low, high))
}
