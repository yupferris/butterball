use super::super::minifb::Window;

#[derive(Default)]
pub struct ProgramState {
    pub app_title: String,

    pub window: Option<Window>
}
