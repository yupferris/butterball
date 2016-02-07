use value::Value;

use super::context::Context;

use time;

pub fn milli_secs(_context: &mut Context, _args: &[Value]) -> Value {
    Value::Integer((time::precise_time_ns() / 1000000) as i32)
}
