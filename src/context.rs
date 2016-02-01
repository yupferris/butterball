use super::minifb::Window;

pub struct Context {
    pub app_title: String,

    pub rng_state: u64,

    pub window: Option<Window>,

    pub width: i32,
    pub height: i32,
    pub back_buffer: Vec<u32>
}

impl Context {
    pub fn new() -> Context {
        Context {
            app_title: String::new(),

            rng_state: 0xffff_ffff_0000_0000,

            window: None,

            width: 0,
            height: 0,
            back_buffer: Vec::new()
        }
    }
}
