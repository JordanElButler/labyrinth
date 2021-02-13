/*
    time to make a little flying camera
*/

use crate::math::{Mat4f};
use crate::transform::{Transform};

pub struct PerspectiveCamera {
    pub transform: Transform,
    pub projection_matrix: Mat4f, 
}

impl PerspectiveCamera {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {

        PerspectiveCamera {
            transform: Transform::identity(),
            projection_matrix: make_projection_matrix(fov, aspect_ratio, near, far),
        }
    }
    
    pub fn view_mat(&self) -> Mat4f {
        self.transform.inverse_mat()
    }
    pub fn view_rot(&self) -> Mat4f {
        let transform = self.transform;

        let mut x_rot = Mat4f::identity();
        x_rot.set(1, 1, transform.rotation.x.cos());
        x_rot.set(1, 2, transform.rotation.x.sin());
        x_rot.set(2, 1, -transform.rotation.x.sin());
        x_rot.set(2, 2, transform.rotation.x.cos());


        let mut y_rot = Mat4f::identity();
        y_rot.set(0, 0, transform.rotation.y.cos());
        y_rot.set(0, 2, transform.rotation.y.sin());
        y_rot.set(2, 0, -transform.rotation.y.sin());
        y_rot.set(2, 2, transform.rotation.y.cos());

        let mut z_rot = Mat4f::identity();
        z_rot.set(0, 0, transform.rotation.z.cos());
        z_rot.set(0, 1, transform.rotation.z.sin());
        z_rot.set(1, 0, -transform.rotation.z.sin());
        z_rot.set(1, 1, transform.rotation.z.cos());

        Mat4f::mult(&x_rot, &Mat4f::mult(&y_rot, &z_rot))
    }
}

pub fn make_projection_matrix(fov: f32, aspect_ratio: f32, n: f32, f: f32) -> Mat4f {
// copied from https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/opengl-perspective-projection-matrix
// but third row (2, x) negated to transform from left-handed to right-handed coordinates

    let scale = (fov * 0.5f32 * std::f32::consts::PI / 180f32).tan() * n;
    let r = aspect_ratio * scale;
    let l = -r;
    let t = scale;
    let b = -t;

    let mut mat = Mat4f::identity();
    mat.set(0, 0, 2f32 * n / (r - l));
    mat.set(1, 1, 2f32 * n / (t - b));
    mat.set(2, 0, (r + l) / (r - l));
    mat.set(2, 1, -(t + b) / (t - b));
    mat.set(2, 2, (f + n) / (f - n));
    mat.set(2, 3, 1f32);
    mat.set(3, 2, -(2f32 * f * n) / (f - n));
    mat.set(3, 3, 0f32);
    mat
}