
use crate::math::Vector3f;
use crate::transform::Transform;

type Color = Vector3f;

pub struct PointLight {
    pub transform: Transform,
    pub color: Color,
    pub power: f32,
}

impl PointLight {
    pub fn new(transform: Transform, color: Color, power: f32) -> Self {
        PointLight {
            transform,
            color: color,
            power: power,
        }
    }

}

pub struct DirectionalLight {
    transform: Transform,
    color: Color,
    power: f32,
}

impl DirectionalLight {
    pub fn new(color: Color, power: f32) -> Self {
        DirectionalLight {
            transform: Transform::identity(),
            color: color,
            power: power,
        }
    }
}