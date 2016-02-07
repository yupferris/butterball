use super::window::Window;

#[derive(Default)]
pub struct Graphics {
    current_buffer: usize
}

impl Graphics {
    pub fn cls(&mut self, window: &mut Window) {
        for pixel in window.back_buffer.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn write_pixel_fast(&mut self, window: &mut Window, x: i32, y: i32, color: i32) {
        window.back_buffer[(y * window.width + x) as usize] = color as u32;
    }
}
