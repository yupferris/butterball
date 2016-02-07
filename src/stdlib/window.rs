use minifb;

#[derive(Default)]
pub struct Window {
    app_title: String,

    window: Option<minifb::Window>,
    // TODO: Make private
    pub width: i32,
    pub height: i32,

    pub back_buffer: Vec<u32> // TODO: Proper buffer
}

impl Window {
    pub fn app_title(&mut self, title: String) {
        self.app_title = title;
        println!("New app title: \"{}\"", self.app_title);
    }

    pub fn graphics(&mut self, width: i32, height: i32, bits: i32, window_mode: i32) {
        println!(
            "Graphics called: {}, {}, {}, {} (ignoring bits and window mode)",
            width,
            height,
            bits,
            window_mode);

        self.window =
            Some(minifb::Window::new(
                &self.app_title,
                width as usize,
                height as usize,
                minifb::Scale::X2).unwrap());

        self.width = width;
        self.height = height;
        self.back_buffer = vec![0; (width * height) as usize];
    }

    pub fn flip(&mut self) {
        println!("WARNING: Flip argument ignored");

        // TODO: It'd be more correct to actually swap between two buffers
        if let Some(ref mut window) = self.window {
            window.update(&self.back_buffer);
        }
    }

    pub fn key_down(&mut self, key: i32) -> bool {
        if let Some(ref mut window) = self.window {
            window.is_key_down(match key {
                1 => minifb::Key::Escape,
                _ => {
                    println!("WARNING: KeyDown called with unrecognized key; defaulting to Escape");

                    minifb::Key::Escape
                }
            })
        } else {
            panic!("KeyDown called without an open window")
        }
    }

    pub fn mouse_down(&mut self, button_index: i32) -> bool {
        if let Some(ref mut window) = self.window {
            window.get_mouse_down(match button_index {
                1 => minifb::MouseButton::Left,
                2 => minifb::MouseButton::Right,
                3 => minifb::MouseButton::Middle,
                _ => panic!("MouseDown called with unrecognized button index: {}", button_index)
            })
        } else {
            panic!("MouseDown called without an open window")
        }
    }

    pub fn mouse_x(&mut self) -> i32 {
        if let Some(ref mut window) = self.window {
            match window.get_mouse_pos(minifb::MouseMode::Clamp) {
                Some((x, _)) => x as i32,
                _ => unreachable!()
            }
        } else {
            panic!("MouseX called without an open window")
        }
    }

    pub fn mouse_y(&mut self) -> i32 {
        if let Some(ref mut window) = self.window {
            match window.get_mouse_pos(minifb::MouseMode::Clamp) {
                Some((_, y)) => y as i32,
                _ => unreachable!()
            }
        } else {
            panic!("MouseY called without an open window")
        }
    }
}
