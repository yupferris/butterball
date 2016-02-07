use ast;
use value::*;
use stdlib::context::*;

use stdlib::window_impls;
use stdlib::graphics_impls;
use stdlib::rng_impls;
use stdlib::image_impls;
use stdlib::math_impls;
use stdlib::time_impls;

use std::fmt;

// TODO: Better name?
pub struct FunctionImpl {
    pub name: String,
    pub function: Box<Fn(&mut Context, &[Value]) -> Value>,
    pub return_type: ValueType
}

impl FunctionImpl {
    pub fn new(
        name: String,
        function: Box<Fn(&mut Context, &[Value]) -> Value>,
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

        FunctionImpl::new(String::from("AppTitle"), Box::new(window_impls::app_title), ValueType::Unit),
        FunctionImpl::new(String::from("Graphics"), Box::new(window_impls::graphics), ValueType::Unit),
        FunctionImpl::new(String::from("Flip"), Box::new(window_impls::flip), ValueType::Unit),
        FunctionImpl::new(String::from("BackBuffer"), Box::new(window_impls::back_buffer), ValueType::Integer),
        FunctionImpl::new(String::from("HidePointer"), Box::new(window_impls::hide_pointer), ValueType::Unit),
        FunctionImpl::new(String::from("KeyDown"), Box::new(window_impls::key_down), ValueType::Bool),
        FunctionImpl::new(String::from("MouseDown"), Box::new(window_impls::mouse_down), ValueType::Bool),
        FunctionImpl::new(String::from("MouseX"), Box::new(window_impls::mouse_x), ValueType::Integer),
        FunctionImpl::new(String::from("MouseY"), Box::new(window_impls::mouse_y), ValueType::Integer),

        FunctionImpl::new(String::from("Cls"), Box::new(graphics_impls::cls), ValueType::Unit),
        FunctionImpl::new(String::from("Color"), Box::new(graphics_impls::color), ValueType::Unit),
        FunctionImpl::new(String::from("Text"), Box::new(graphics_impls::text), ValueType::Unit),
        FunctionImpl::new(String::from("DrawBlock"), Box::new(graphics_impls::draw_block), ValueType::Unit),
        FunctionImpl::new(String::from("SetBuffer"), Box::new(graphics_impls::set_buffer), ValueType::Unit),
        FunctionImpl::new(String::from("LockBuffer"), Box::new(graphics_impls::lock_buffer), ValueType::Unit),
        FunctionImpl::new(String::from("UnlockBuffer"), Box::new(graphics_impls::unlock_buffer), ValueType::Unit),
        FunctionImpl::new(String::from("WritePixelFast"), Box::new(graphics_impls::write_pixel_fast), ValueType::Unit),

        FunctionImpl::new(String::from("SeedRnd"), Box::new(rng_impls::seed_rnd), ValueType::Unit),
        FunctionImpl::new(String::from("Rand"), Box::new(rng_impls::rand), ValueType::Integer),
        FunctionImpl::new(String::from("Rnd"), Box::new(rng_impls::rnd), ValueType::Float),

        FunctionImpl::new(String::from("CreateImage"), Box::new(image_impls::create_image), ValueType::Integer),
        FunctionImpl::new(String::from("FreeImage"), Box::new(image_impls::free_image), ValueType::Unit),
        FunctionImpl::new(String::from("ImageBuffer"), Box::new(image_impls::image_buffer), ValueType::Integer),
        FunctionImpl::new(String::from("MidHandle"), Box::new(image_impls::mid_handle), ValueType::Unit),

        FunctionImpl::new(String::from("Abs"), Box::new(math_impls::abs), ValueType::Float),
        FunctionImpl::new(String::from("Sin"), Box::new(math_impls::sin), ValueType::Float),
        FunctionImpl::new(String::from("Cos"), Box::new(math_impls::cos), ValueType::Float),
        FunctionImpl::new(String::from("Sqr"), Box::new(math_impls::sqr), ValueType::Float),

        FunctionImpl::new(String::from("MilliSecs"), Box::new(time_impls::milli_secs), ValueType::Integer)]
}

fn float_cast(_context: &mut Context, args: &[Value]) -> Value {
    let arg = &args[0];
    Value::Float(match arg {
        &Value::Integer(x) => x as f32,
        &Value::Float(x) => x,
        _ => panic!("Unable to cast value to float: {:?}", arg)
    })
}

pub fn build_un_op_impls_table() -> Vec<((ast::Op, ValueType), FunctionImpl)> {
    vec![
        ((ast::Op::Not, ValueType::Bool), FunctionImpl::new(String::from("un_op_not_bool"), Box::new(un_op_not_bool), ValueType::Bool)),

        ((ast::Op::Neg, ValueType::Integer), FunctionImpl::new(String::from("un_op_neg_int"), Box::new(un_op_neg_int), ValueType::Integer)),
        ((ast::Op::Neg, ValueType::Float), FunctionImpl::new(String::from("un_op_neg_float"), Box::new(un_op_neg_float), ValueType::Float))]
}

fn un_op_not_bool(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(!args[0].as_bool())
}

fn un_op_neg_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(-args[0].as_integer())
}

fn un_op_neg_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(-args[0].as_float())
}

pub fn build_bin_op_impls_table() -> Vec<((ast::Op, ValueType, ValueType), FunctionImpl)> {
    vec![
        ((ast::Op::LtEq, ValueType::Integer, ValueType::Integer),
         FunctionImpl::new(String::from("bin_op_lt_eq_int_int"), Box::new(bin_op_lt_eq_int_int), ValueType::Bool)),
        ((ast::Op::LtEq, ValueType::Integer, ValueType::Float),
         FunctionImpl::new(String::from("bin_op_lt_eq_int_float"), Box::new(bin_op_lt_eq_int_float), ValueType::Bool)),

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

fn bin_op_lt_eq_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_integer() <= args[1].as_integer())
}

fn bin_op_lt_eq_int_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool((args[0].as_integer() as f32) <= args[1].as_float())
}

fn bin_op_gt_eq_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_integer() >= args[1].as_integer())
}

fn bin_op_gt_eq_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_float() >= (args[1].as_integer() as f32))
}

fn bin_op_lt_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_integer() < args[1].as_integer())
}

fn bin_op_lt_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_float() < (args[1].as_integer() as f32))
}

fn bin_op_gt_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_integer() > args[1].as_integer())
}

fn bin_op_gt_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_float() > (args[1].as_integer() as f32))
}

fn bin_op_eq_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_integer() == args[1].as_integer())
}

fn bin_op_and_bool_bool(_context: &mut Context, args: &[Value]) -> Value {
    Value::Bool(args[0].as_bool() && args[1].as_bool())
}

fn bin_op_add_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(args[0].as_integer() + args[1].as_integer())
}

fn bin_op_add_int_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float((args[0].as_integer() as f32) + args[1].as_float())
}

fn bin_op_add_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float() + (args[1].as_integer() as f32))
}

fn bin_op_add_float_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float() + args[1].as_float())
}

fn bin_op_add_int_string(_context: &mut Context, args: &[Value]) -> Value {
    Value::String(format!("{}{}", args[0].as_integer(), args[1].as_string()))
}

fn bin_op_sub_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(args[0].as_integer() - args[1].as_integer())
}

fn bin_op_sub_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float() - (args[1].as_integer() as f32))
}

fn bin_op_sub_float_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float() - args[1].as_float())
}

fn bin_op_mul_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(args[0].as_integer() * args[1].as_integer())
}

fn bin_op_mul_int_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float((args[0].as_integer() as f32) * args[1].as_float())
}

fn bin_op_mul_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float() * (args[1].as_integer() as f32))
}

fn bin_op_mul_float_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float() * args[1].as_float())
}

fn bin_op_div_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(args[0].as_integer() / args[1].as_integer())
}

fn bin_op_div_float_float(_context: &mut Context, args: &[Value]) -> Value {
    Value::Float(args[0].as_float() / args[1].as_float())
}

fn bin_op_shl_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(args[0].as_integer() << args[1].as_integer())
}

fn bin_op_shl_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer((args[0].as_float() as i32) << args[1].as_integer())
}

fn bin_op_shr_int_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer(args[0].as_integer() >> args[1].as_integer())
}

fn bin_op_shr_float_int(_context: &mut Context, args: &[Value]) -> Value {
    Value::Integer((args[0].as_float() as i32) >> args[1].as_integer())
}
