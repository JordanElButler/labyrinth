use crate::math::Vector2f;

pub struct InputState {
    pub mouse_state: MouseState,
    pub key_state: KeyState,
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
    pub left: bool,
    pub right: bool,
    pub x: f32,
    pub y: f32,
    pub px: f32,
    pub py: f32,
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
    pub fn movement(&self) -> bool {
        self.px != self.x && self.py != self.y
    }
    pub fn get_direction_normal(&self) -> Vector2f {
        let mut v = Vector2f::new(self.x - self.px, self.y - self.py);
        v.normalize();
        v
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
    pub fn key_down(&self, key: &sdl2::keyboard::Keycode) -> bool {
        self.key_map.contains(&key)
    }
}