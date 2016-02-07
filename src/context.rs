use stdlib::window::Window;
use stdlib::graphics::Graphics;

pub struct Context {
    pub rng_state: u64,

    pub window: Window,
    pub graphics: Graphics
}

impl Context {
    pub fn new() -> Context {
        Context {
            rng_state: 0xffff_ffff_0000_0000,

            window: Window::default(),
            graphics: Graphics::default()
        }
    }
}
