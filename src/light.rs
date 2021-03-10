
use crate::transform::Transform;
use crate::math::Vector3f;


pub enum LightType {
    PointLight,
    DirectionalLight,
}

pub struct Light {
    pub light_type: LightType,
    pub transform: Transform,
    pub color: Vector3f,
    pub power: f32,
}

impl Light {
    pub fn new_point_light(transform: Transform, color: Vector3f, power: f32) -> Self {
        Light {
            light_type: LightType::PointLight,
            transform,
            color,
            power,
        }
    }
    pub fn new_directional_light(transform: Transform, color: Vector3f, power: f32) -> Self {
        Light {
            light_type: LightType::DirectionalLight,
            transform,
            color,
            power,
        }
    }
    pub fn get_position(&self) -> &Vector3f {
        &self.transform.translation
    }
    pub fn get_color(&self) -> &Vector3f {
        &self.color
    }
}
