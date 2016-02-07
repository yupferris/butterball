use std::ops::{Index, IndexMut};

pub struct Buffer {
    pub width: i32,
    pub height: i32,

    pub data: Box<[u32]>
}

impl Buffer {
    pub fn new(width: i32, height: i32) -> Buffer {
        Buffer {
            width: width,
            height: height,

            data: vec![0; (width * height) as usize].into_boxed_slice()
        }
    }
}

#[derive(Default)]
pub struct Buffers {
    buffers: Vec<Option<Buffer>>
}

impl Buffers {
    pub fn alloc(&mut self, width: i32, height: i32) -> usize {
        let buffer = Buffer::new(width, height);

        for (i, existing_buffer) in self.buffers.iter_mut().enumerate() {
            if existing_buffer.is_none() {
                *existing_buffer = Some(buffer);
                return i;
            }
        }

        let ret = self.buffers.len();
        self.buffers.push(Some(buffer));

        ret
    }

    pub fn free(&mut self, handle: usize) {
        self.buffers[handle] = None;
    }
}

impl Index<usize> for Buffers {
    type Output = Buffer;

    fn index<'a>(&'a self, index: usize) -> &'a Buffer {
        match &self.buffers[index] {
            &Some(ref buffer) => buffer,
            _ => panic!("Invalid buffer index: {}", index)
        }
    }
}

impl IndexMut<usize> for Buffers {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Buffer {
        match &mut self.buffers[index] {
            &mut Some(ref mut buffer) => buffer,
            _ => panic!("Invalid buffer index: {}", index)
        }
    }
}
