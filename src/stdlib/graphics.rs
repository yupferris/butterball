use super::buffers::Buffers;

#[derive(Default)]
pub struct Graphics {
    current_buffer_handle: usize
}

impl Graphics {
    pub fn cls(&mut self, buffers: &mut Buffers) {
        for pixel in buffers[self.current_buffer_handle].data.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn set_buffer(&mut self, buffer: i32) {
        self.current_buffer_handle = buffer as usize;
    }

    pub fn write_pixel_fast(&mut self, buffers: &mut Buffers, x: i32, y: i32, color: i32, buffer_handle: Option<i32>) {
        let buffer = &mut buffers[buffer_handle.map_or(self.current_buffer_handle, |x| x as usize)];
        buffer.data[(y * buffer.width + x) as usize] = color as u32;
    }
}
