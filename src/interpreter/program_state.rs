use super::super::minifb::Window;

#[derive(Default)]
pub struct ProgramState {
    pub app_title: String,

    pub window: Option<Window>,

    pub data_pointer: usize,

    pub width: i32,
    pub height: i32,
    pub back_buffer: Vec<u32>
}
