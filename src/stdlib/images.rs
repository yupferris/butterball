use std::ops::{Index, IndexMut};

pub struct Image {
    pub buffer_handle: usize,
    pub mid_handle: bool
}

impl Image {
    pub fn new(buffer_handle: usize) -> Image {
        Image {
            buffer_handle: buffer_handle,
            mid_handle: false
        }
    }
}

#[derive(Default)]
pub struct Images {
    images: Vec<Option<Image>>
}

// TODO: Can probably reuse some code between this and buffers::Buffers; the alloc/handle mechanism is the same
impl Images {
    pub fn alloc(&mut self, buffer_handle: usize) -> usize {
        let image = Image::new(buffer_handle);

        for (i, existing_image) in self.images.iter_mut().enumerate() {
            if existing_image.is_none() {
                *existing_image = Some(image);
                return i;
            }
        }

        let ret = self.images.len();
        self.images.push(Some(image));

        ret
    }

    pub fn free(&mut self, handle: usize) {
        self.images[handle] = None;
    }
}

impl Index<usize> for Images {
    type Output = Image;

    fn index<'a>(&'a self, index: usize) -> &'a Image {
        match &self.images[index] {
            &Some(ref image) => image,
            _ => panic!("Invalid image index: {}", index)
        }
    }
}

impl IndexMut<usize> for Images {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Image {
        match &mut self.images[index] {
            &mut Some(ref mut image) => image,
            _ => panic!("Invalid image index: {}", index)
        }
    }
}
