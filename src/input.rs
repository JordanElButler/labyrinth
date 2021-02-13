
pub struct InputState {
    mouse_state: MouseState,
    key_state: KeyState,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mouse_state: MouseState::new(),
            key_state: KeyState::new(),
        }
    }
    pub fn update(&mut self, event: &sdl2::event::Event) {
        self.mouse_state.update(event);
        self.key_state.update(event);
    }
}

#[derive(Debug)]
pub struct MouseState {
    left: bool,
    right: bool,
    x: f32,
    y: f32,
    px: f32,
    py: f32,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            left: false,
            right: false,
            x: 0f32,
            y: 0f32,
            px: 0f32,
            py: 0f32,
        }
    }
    pub fn update(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::MouseMotion{x, y, ..} => {
                self.px = self.x;
                self.py = self.y;
                self.x = *x as f32;
                self.y = *y as f32;
            },
            sdl2::event::Event::MouseButtonDown{mouse_btn, ..} => {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => self.left = true,
                    sdl2::mouse::MouseButton::Right => self.right = true,
                    _ => ()
                }
            },
            sdl2::event::Event::MouseButtonUp{mouse_btn, ..} => {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => self.left = false,
                    sdl2::mouse::MouseButton::Right => self.right = false,
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

use std::collections::HashSet;

#[derive(Debug)]
pub struct KeyState {
    key_map: HashSet<sdl2::keyboard::Keycode>,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            key_map: HashSet::new(),
        }
    }
    pub fn update(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown{ keycode: Some(key), ..} => {
                self.key_map.insert(*key);
            },
            sdl2::event::Event::KeyUp{ keycode: Some(key), ..} => {
                self.key_map.remove(key);
            },
            _ => ()
        }
    }
}