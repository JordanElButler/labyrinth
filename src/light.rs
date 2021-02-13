
use crate::transform::Transform;
use crate::color::Color;

pub enum LightType {
    PointLight,
    DirectionalLight,
}

pub struct Light {
    pub light_type: LightType,
    pub transform: Transform,
    pub color: Color,
    pub power: f32,
}

impl Light {
    pub fn new_point_light(transform: Transform, color: Color, power: f32) -> Self {
        Light {
            light_type: LightType::PointLight,
            transform,
            color: color,
            power: power,
        }
    }
    pub fn new_directional_light(transform: Transform, color: Color, power: f32) -> Self {
        Light {
            light_type: LightType::DirectionalLight,
            transform,
            color,
            power,
        }
    }
}
