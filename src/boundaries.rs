
pub struct AABB {
    x: f32,
    y: f32,
    z: f32,
}

impl AABB {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        AABB {
            x,
            y,
            z,
        }
    }
}

pub struct Cylinder {
    y: f32,
    r: f32,
}

impl Cylinder {
    pub fn new(y: f32, r: f32) -> Self {
        Cylinder {
            y,
            r,
        }
    }
}

pub enum IntersectionResult {
    None,
    Displacement(Vector3f),
}

use crate::math::Vector3f;
pub fn AABB_AABB_intersection(cyl: &Cylinder, cpos: &Vector3f, aabb: &AABB, apos: &Vector3f) -> IntersectionResult {
    let vdiff = Vector3f::sub(cpos, apos);

    if vdiff.y > cyl.y + aabb.y || !(vdiff.x < cyl.r + aabb.x && vdiff.z < cyl.r + aabb.z) {
        return IntersectionResult::None;
    } else {
        return IntersectionResult::Displacement(vdiff);
    }
}