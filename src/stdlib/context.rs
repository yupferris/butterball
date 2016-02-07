use super::window::Window;
use super::graphics::Graphics;
use super::rng::Rng;
use super::buffers::Buffers;
use super::images::Images;

pub struct Context {
    pub window: Window,
    pub graphics: Graphics,
    pub rng: Rng,
    pub buffers: Buffers,
    pub images: Images
}

impl Context {
    pub fn new() -> Context {
        Context {
            window: Window::default(),
            graphics: Graphics::default(),
            rng: Rng::new(),
            buffers: Buffers::default(),
            images: Images::default()
        }
    }
}
