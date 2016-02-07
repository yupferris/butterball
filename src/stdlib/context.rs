use super::window::Window;
use super::graphics::Graphics;
use super::rng::Rng;

pub struct Context {
    pub window: Window,
    pub graphics: Graphics,
    pub rng: Rng
}

impl Context {
    pub fn new() -> Context {
        Context {
            window: Window::default(),
            graphics: Graphics::default(),
            rng: Rng::new()
        }
    }
}
