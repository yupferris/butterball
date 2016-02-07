use super::window::Window;
use super::graphics::Graphics;
use super::rng::Rng;
use super::buffers::Buffers;

pub struct Context {
    pub window: Window,
    pub graphics: Graphics,
    pub rng: Rng,
    pub buffers: Buffers
}

impl Context {
    pub fn new() -> Context {
        Context {
            window: Window::default(),
            graphics: Graphics::default(),
            rng: Rng::new(),
            buffers: Buffers::default()
        }
    }
}
