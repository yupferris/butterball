use super::super::minifb::Window;

pub struct ProgramState {
    pub app_title: String,

    pub rng_state: u64,

    pub window: Option<Window>,

    pub data_pointer: usize,

    pub width: i32,
    pub height: i32,
    pub back_buffer: Vec<u32>
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            app_title: String::new(),

            rng_state: 0xffff_ffff_0000_0000,

            window: None,

            data_pointer: 0,

            width: 0,
            height: 0,
            back_buffer: Vec::new()
        }
    }
}
