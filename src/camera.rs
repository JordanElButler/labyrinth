/*
    time to make a little flying camera
*/
use crate::math::{Mat4f};
use crate::transform::{Transform};

pub enum CameraType {
    PerspectiveCamera{ projection_matrix: Mat4f },
    OrthographicCamera{ orthographic_matrix: Mat4f },
}

pub struct Camera {
    pub transform: Transform,
    camera_type: CameraType,
}

impl Camera {
    pub fn new_perspective_camera(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {

        Camera {
            transform: Transform::identity(),
            camera_type: CameraType::PerspectiveCamera{ projection_matrix: make_projection_matrix(fov, aspect_ratio, near, far) },
        }
    }
    pub fn new_orthographic_camera(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Camera {
            transform: Transform::identity(),
            camera_type: CameraType::OrthographicCamera{ orthographic_matrix: make_orthographic_matrix(left, right, bottom, top, near, far) },
        }
    }
    pub fn proj_mat(&self) -> &Mat4f {
        match &self.camera_type {
            CameraType::PerspectiveCamera{ projection_matrix: mat} => {
                mat
            },
            CameraType::OrthographicCamera{ orthographic_matrix: mat} => {
                mat
            }
            _ => panic!("No proj matrix?")
        }
    }
    pub fn view_mat(&self) -> Mat4f {
        self.transform.inverse_mat()
    }
    pub fn view_rot(&self) -> Mat4f {
        let x_rot = Mat4f::rotation_x(-self.transform.rotation.x);
        let y_rot = Mat4f::rotation_y(-self.transform.rotation.y);
        let z_rot = Mat4f::rotation_z(-self.transform.rotation.z);

        Mat4f::mult(&x_rot, &Mat4f::mult(&y_rot, &z_rot))
    }
    
}


pub fn make_projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4f {
// copied from https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/opengl-perspective-projection-matrix
// but third row (2, x) negated to transform from left-handed to right-handed coordinates
   
    let scale = (fov * 0.5f32 * std::f32::consts::PI / 180f32).tan() * near;
    let r = aspect_ratio * scale;
    let l = -r;
    let t = scale;
    let b = -t;

    let mut mat = Mat4f::identity();
    mat.set(0, 0, 2f32 * near / (r - l));
    mat.set(1, 1, 2f32 * near / (t - b));
    mat.set(0, 2, -(r + l) / (r - l));
    mat.set(1, 2, (t + b) / (t - b));
    mat.set(2, 2, -(far + near) / (far - near));
    mat.set(3, 2, -1f32);
    mat.set(2, 3, -(2f32 * far * near) / (far - near));
    mat.set(3, 3, 0f32);
    mat

}

fn make_orthographic_matrix(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4f {
    let x_ortho = 2f32 / (right - left);
    let y_ortho = 2f32 / (top - bottom);
    let z_ortho = -2f32 / (far - near);

    let x_trans = -(right + left) / (right - left);
    let y_trans = -(top + bottom) / (top - bottom);
    let z_trans = -(far + near) / (far - near);

    let mut mat = Mat4f::identity();
    mat.set(0, 0, x_ortho);
    mat.set(1, 1, y_ortho);
    mat.set(2, 2, z_ortho);
    mat.set(0, 3, x_trans);
    mat.set(1, 3, y_trans);
    mat.set(2, 3, z_trans);
    mat
}