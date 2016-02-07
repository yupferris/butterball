use super::buffers::Buffers;
use super::images::Images;

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

    pub fn draw_block(&mut self, images: &Images, buffers: &mut Buffers, image_handle: i32, x: i32, y: i32) {
        let image = &images[image_handle as usize];

        let (image_width, image_height, x_offset, y_offset) = {
            let image_buffer = &buffers[image.buffer_handle];

            let x_offset = if image.mid_handle { x - image_buffer.width / 2 } else { x };
            let y_offset = if image.mid_handle { y - image_buffer.height / 2 } else { y };

            (image_buffer.width, image_buffer.height, x_offset, y_offset)
        };

        let (target_width, target_height) = {
            let target_buffer = &mut buffers[self.current_buffer_handle];

            (target_buffer.width, target_buffer.height)
        };

        for iy in 0..image_height {
            let py = y_offset + iy;
            if py >= 0 && py < target_height {
                for ix in 0..image_width {
                    let px = x_offset + ix;
                    if px >= 0 && px < target_width {
                        let pixel = buffers[image.buffer_handle].data[(iy * image_width + ix) as usize];
                        buffers[self.current_buffer_handle].data[(py * target_width + px) as usize] = pixel
                    }
                }
            }
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
