
use crate::math::{Vector3f};

// represents an unbounded color, needs tone-mapping operator
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
}