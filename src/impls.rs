use super::minifb::{Window, Scale, Key, MouseButton, MouseMode};
use super::time;

use super::ast;
use super::value::*;
use super::context::*;

use std::fmt;

use std::f32::consts;

// TODO: Better name?
pub struct FunctionImpl {
    pub name: String,
    pub function: Box<Fn(&mut Context, &Vec<Value>) -> Value>,
    pub return_type: ValueType
}

impl FunctionImpl {
    pub fn new(
        name: String,
        function: Box<Fn(&mut Context, &Vec<Value>) -> Value>,
        return_type: ValueType) -> FunctionImpl {

        FunctionImpl {
            name: name,
            function: function,
            return_type: return_type
        }
    }
}

impl fmt::Debug for FunctionImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FunctionImpl {{ name: {}, return_type: {:?} }}", self.name, self.return_type)
    }
}

// TODO: Better name
pub fn build_impls_table() -> Vec<FunctionImpl> {
    vec![
        FunctionImpl::new(String::from("Float"), Box::new(float_cast), ValueType::Float),

        FunctionImpl::new(String::from("Abs"), Box::new(abs), ValueType::Float),

        FunctionImpl::new(String::from("Sin"), Box::new(sin), ValueType::Float),
        FunctionImpl::new(String::from("Cos"), Box::new(cos), ValueType::Float),

        FunctionImpl::new(String::from("AppTitle"), Box::new(app_title), ValueType::Unit),
        FunctionImpl::new(String::from("Graphics"), Box::new(graphics), ValueType::Unit),

        FunctionImpl::new(String::from("SetBuffer"), Box::new(set_buffer), ValueType::Unit),
        FunctionImpl::new(String::from("BackBuffer"), Box::new(back_buffer), ValueType::Integer),

        FunctionImpl::new(String::from("LockBuffer"), Box::new(lock_buffer), ValueType::Unit),
        FunctionImpl::new(String::from("UnlockBuffer"), Box::new(unlock_buffer), ValueType::Unit),

        FunctionImpl::new(String::from("WritePixelFast"), Box::new(write_pixel_fast), ValueType::Unit),

        FunctionImpl::new(String::from("HidePointer"), Box::new(hide_pointer), ValueType::Unit),

        FunctionImpl::new(String::from("SeedRnd"), Box::new(seed_rnd), ValueType::Unit),
        FunctionImpl::new(String::from("Rand"), Box::new(rand), ValueType::Integer),

        FunctionImpl::new(String::from("MilliSecs"), Box::new(milli_secs), ValueType::Integer),

        FunctionImpl::new(String::from("KeyDown"), Box::new(key_down), ValueType::Bool),

        FunctionImpl::new(String::from("MouseDown"), Box::new(mouse_down), ValueType::Bool),
        FunctionImpl::new(String::from("MouseX"), Box::new(mouse_x), ValueType::Integer),
        FunctionImpl::new(String::from("MouseY"), Box::new(mouse_y), ValueType::Integer),

        FunctionImpl::new(String::from("Cls"), Box::new(cls), ValueType::Unit),
        FunctionImpl::new(String::from("Flip"), Box::new(flip), ValueType::Unit),

        FunctionImpl::new(String::from("Color"), Box::new(color), ValueType::Unit),
        FunctionImpl::new(String::from("Text"), Box::new(text), ValueType::Unit)]
}

fn float_cast(_: &mut Context, args: &Vec<Value>) -> Value {
    let arg = &args[0];
    Value::Float(match arg {
        &Value::Integer(x) => x as f32,
        &Value::Float(x) => x,
        _ => panic!("Unable to cast value to float: {:?}", arg)
    })
}

// TODO: Support type overloads
fn abs(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(args[0].as_float().abs())
}

fn sin(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(degrees_to_radians(args[0].cast_to_float().as_float()).sin())
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees / 180.0 * consts::PI
}

fn cos(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(degrees_to_radians(args[0].cast_to_float().as_float()).cos())
}

fn app_title(context: &mut Context, args: &Vec<Value>) -> Value {
    context.app_title = args[0].as_string();
    println!("New app title: \"{}\"", context.app_title);

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

    context.window =
        Some(Window::new(
            &context.app_title,
            width as usize,
            height as usize,
            Scale::X2).unwrap());

    context.width = width;
    context.height = height;
    context.back_buffer = vec![0; (width * height) as usize];

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

    context.back_buffer[(y * context.width + x) as usize] = color;

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

fn seed_rnd(context: &mut Context, args: &Vec<Value>) -> Value {
    context.rng_state = 0xffff_ffff_0000_0000 | (args[0].as_integer() as u64);

    Value::Unit
}

fn rand(context: &mut Context, args: &Vec<Value>) -> Value {
    // xorshift* prng
    let mut x = context.rng_state;
    x ^= x >> 12;
    x ^= x >> 25;
    x ^= x >> 27;
    x *= 2685821657736338717;
    context.rng_state = x;

    let (low, high) = match args.len() {
        1 => (0, args[0].as_integer()),
        2 => (args[0].as_integer(), args[1].as_integer()),
        _ => panic!("Invalid number of arguments to Rand: {}", args.len())
    };
    let range = high - low + 1;

    Value::Integer(((x as i32) % range) + low)
}

fn milli_secs(_: &mut Context, _: &Vec<Value>) -> Value {
    Value::Integer((time::precise_time_ns() / 1000000) as i32)
}

fn key_down(context: &mut Context, args: &Vec<Value>) -> Value {
    if let Some(ref mut window) = context.window {
        Value::Bool(window.is_key_down(match args[0].as_integer() {
            1 => Key::Escape,
            _ => {
                println!("WARNING: KeyDown called with unrecognized key; defaulting to Escape");

                Key::Escape
            }
        }))
    } else {
        panic!("KeyDown called without an open window")
    }
}

fn mouse_down(context: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(if let Some(ref mut window) = context.window {
        let button_index = args[0].as_integer();
        window.get_mouse_down(match button_index {
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            _ => panic!("MouseDown called with unrecognized button index: {}", button_index)
        })
    } else {
        panic!("MouseDown called without an open window")
    })
}

fn mouse_x(context: &mut Context, _args: &Vec<Value>) -> Value {
    Value::Integer(if let Some(ref mut window) = context.window {
        match window.get_mouse_pos(MouseMode::Clamp) {
            Some((x, _)) => x as i32,
            _ => unreachable!()
        }
    } else {
        panic!("MouseX called without an open window")
    })
}

fn mouse_y(context: &mut Context, _args: &Vec<Value>) -> Value {
    Value::Integer(if let Some(ref mut window) = context.window {
        match window.get_mouse_pos(MouseMode::Clamp) {
            Some((_, y)) => y as i32,
            _ => unreachable!()
        }
    } else {
        panic!("MouseY called without an open window")
    })
}

fn cls(context: &mut Context, _: &Vec<Value>) -> Value {
    for pixel in context.back_buffer.iter_mut() {
        *pixel = 0;
    }

    Value::Unit
}

fn flip(context: &mut Context, _: &Vec<Value>) -> Value {
    println!("WARNING: Flip argument ignored");

    // TODO: It'd be more correct to actually swap between two buffers
    let buffer = &context.back_buffer;
    if let Some(ref mut window) = context.window {
        window.update(buffer);
    }

    Value::Unit
}

fn color(_: &mut Context, _: &Vec<Value>) -> Value {
    println!("Color called (and ignored)");

    Value::Unit
}

fn text(_: &mut Context, args: &Vec<Value>) -> Value {
    println!("Text called; drawing was ignored: {}, {}, {:?}", args[0].as_integer(), args[1].as_integer(), args[2].as_string());

    Value::Unit
}

pub fn build_un_op_impls_table() -> Vec<((ast::Op, ValueType), FunctionImpl)> {
    vec![
        ((ast::Op::Not, ValueType::Bool), FunctionImpl::new(String::from("un_op_not_bool"), Box::new(un_op_not_bool), ValueType::Bool)),

        ((ast::Op::Neg, ValueType::Integer), FunctionImpl::new(String::from("un_op_neg_int"), Box::new(un_op_neg_int), ValueType::Integer)),
        ((ast::Op::Neg, ValueType::Float), FunctionImpl::new(String::from("un_op_neg_float"), Box::new(un_op_neg_float), ValueType::Float))]
}

fn un_op_not_bool(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(!args[0].as_bool())
}

fn un_op_neg_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Integer(-args[0].as_integer())
}

fn un_op_neg_float(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Float(-args[0].as_float())
}

pub fn build_bin_op_impls_table() -> Vec<((ast::Op, ValueType, ValueType), FunctionImpl)> {
    vec![
        ((ast::Op::LtEq, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_lt_eq_int_int"), Box::new(bin_op_lt_eq_int_int), ValueType::Bool)),

        ((ast::Op::GtEq, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_gt_eq_int_int"), Box::new(bin_op_gt_eq_int_int), ValueType::Bool)),
        ((ast::Op::GtEq, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_gt_eq_float_int"), Box::new(bin_op_gt_eq_float_int), ValueType::Bool)),

        ((ast::Op::Lt, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_lt_int_int"), Box::new(bin_op_lt_int_int), ValueType::Bool)),
        ((ast::Op::Lt, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_lt_float_int"), Box::new(bin_op_lt_float_int), ValueType::Bool)),

        ((ast::Op::Gt, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_gt_int_int"), Box::new(bin_op_gt_int_int), ValueType::Bool)),
        ((ast::Op::Gt, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_gt_float_int"), Box::new(bin_op_gt_float_int), ValueType::Bool)),

        ((ast::Op::Eq, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_eq_int_int"), Box::new(bin_op_eq_int_int), ValueType::Bool)),

        ((ast::Op::And, ValueType::Bool, ValueType::Bool),
         FunctionImpl::new(String::from("bin_op_and_bool_bool"), Box::new(bin_op_and_bool_bool), ValueType::Bool)),

        ((ast::Op::Add, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_add_int_int"), Box::new(bin_op_add_int_int), ValueType::Integer)),
        ((ast::Op::Add, ValueType::Integer, ValueType::Float),
         FunctionImpl::new(String::from("bin_op_add_int_float"), Box::new(bin_op_add_int_float), ValueType::Float)),
        ((ast::Op::Add, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_add_float_int"), Box::new(bin_op_add_float_int), ValueType::Float)),
        ((ast::Op::Add, ValueType::Float, ValueType::Float),
         FunctionImpl::new(String::from("bin_op_add_float_float"), Box::new(bin_op_add_float_float), ValueType::Float)),

        ((ast::Op::Add, ValueType::Integer, ValueType::String),
         FunctionImpl::new(String::from("bin_op_add_int_string"), Box::new(bin_op_add_int_string), ValueType::String)),

        ((ast::Op::Sub, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_sub_int_int"), Box::new(bin_op_sub_int_int), ValueType::Integer)),
        ((ast::Op::Sub, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_sub_float_int"), Box::new(bin_op_sub_float_int), ValueType::Float)),
        ((ast::Op::Sub, ValueType::Float, ValueType::Float),
         FunctionImpl::new(String::from("bin_op_sub_float_float"), Box::new(bin_op_sub_float_float), ValueType::Float)),

        ((ast::Op::Mul, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_mul_int_int"), Box::new(bin_op_mul_int_int), ValueType::Integer)),
        ((ast::Op::Mul, ValueType::Integer, ValueType::Float),
         FunctionImpl::new(String::from("bin_op_mul_int_float"), Box::new(bin_op_mul_int_float), ValueType::Float)),
        ((ast::Op::Mul, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_mul_float_int"), Box::new(bin_op_mul_float_int), ValueType::Float)),
        ((ast::Op::Mul, ValueType::Float, ValueType::Float),
         FunctionImpl::new(String::from("bin_op_mul_float_float"), Box::new(bin_op_mul_float_float), ValueType::Float)),

        ((ast::Op::Div, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_div_int_int"), Box::new(bin_op_div_int_int), ValueType::Integer)),
        ((ast::Op::Div, ValueType::Float, ValueType::Float),
         FunctionImpl::new(String::from("bin_op_div_float_float"), Box::new(bin_op_div_float_float), ValueType::Float)),

        ((ast::Op::Shl, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_shl_int_int"), Box::new(bin_op_shl_int_int), ValueType::Integer)),
        ((ast::Op::Shl, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_shl_float_int"), Box::new(bin_op_shl_float_int), ValueType::Integer)),

        ((ast::Op::Shr, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_shr_int_int"), Box::new(bin_op_shr_int_int), ValueType::Integer)),
        ((ast::Op::Shr, ValueType::Float, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_shr_float_int"), Box::new(bin_op_shr_float_int), ValueType::Integer))]
}

fn bin_op_lt_eq_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_integer() <= args[1].as_integer())
}

fn bin_op_gt_eq_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_integer() >= args[1].as_integer())
}

fn bin_op_gt_eq_float_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_float() >= (args[1].as_integer() as f32))
}

fn bin_op_lt_int_int(_: &mut Context, args: &Vec<Value>) -> Value {
    Value::Bool(args[0].as_integer() < args[1].as_integer())
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
    Value::String(format!("{}{}", args[0].as_integer(), args[1].as_string()))
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
