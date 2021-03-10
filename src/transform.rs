use crate::math::{Vector3f, Mat4f};

#[derive(Debug, Copy, Clone)]
pub struct Basis {
    pub v1: Vector3f,
    pub v2: Vector3f,
    pub v3: Vector3f,
}

impl Basis {
    pub fn new(v1: Vector3f, v2: Vector3f, v3: Vector3f) -> Self {
        Basis {
            v1,
            v2,
            v3,
        }
    }
    pub fn transform_basis(&self, transform: &Transform) -> Basis {
        let model_rot = transform.model_rot();

        Basis::to_basis(&Mat4f::mult(&model_rot, &Basis::to_mat(&self)))
    }
    pub fn to_basis(m: &Mat4f) -> Basis {
        Basis {
            v1: Vector3f::new(m.get(0, 0), m.get(1, 0), m.get(2, 0)),
            v2: Vector3f::new(m.get(0, 1), m.get(1, 1), m.get(2, 1)),
            v3: Vector3f::new(m.get(0, 2), m.get(1, 2), m.get(2, 2)),
        }
    }
    pub fn to_mat(&self) -> Mat4f {
        let mut mat = Mat4f::identity();
        mat.set(0, 0, self.v1.x); mat.set(1, 0, self.v1.y); mat.set(2, 0, self.v1.z);
        mat.set(0, 1, self.v2.x); mat.set(1, 1, self.v2.y); mat.set(2, 1, self.v2.z);
        mat.set(0, 2, self.v3.x); mat.set(1, 2, self.v3.y); mat.set(2, 2, self.v3.z);
        mat
    }
    pub fn scale_and_add(&self, v: &Vector3f) -> Vector3f {
        let mut v1 = self.v1.copy();
        let mut v2 = self.v2.copy();
        let mut v3 = self.v3.copy();

        v1.scale(v.x);
        v2.scale(v.y);
        v3.scale(v.z);

        v2.add_to(&v3);
        v1.add_to(&v2);
        v1

    }
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vector3f,
    pub rotation: Vector3f,
    pub scale: Vector3f,
}

impl Transform {

    pub fn identity() -> Self {
        Transform {
            translation: Vector3f::new(0f32, 0f32, 0f32),
            rotation: Vector3f::new(0f32, 0f32, 0f32),
            scale: Vector3f::new(1f32, 1f32, 1f32),
        }
    }
    pub fn model_rot(&self) -> Mat4f {
        let z_rot = Mat4f::rotation_z(self.rotation.z);
        let y_rot = Mat4f::rotation_y(self.rotation.y);
        let x_rot = Mat4f::rotation_x(self.rotation.x);

        Mat4f::mult(&z_rot, &Mat4f::mult(&y_rot, &x_rot))
    }
    pub fn model_mat(&self) -> Mat4f {
        let scale = Mat4f::scale(self.scale);
        let rot = self.model_rot();
        let trans = Mat4f::translation(self.translation);
        
        Mat4f::mult(&trans, &Mat4f::mult(&rot, &scale))

    }

    // inverse of transform matrix
    pub fn inverse_mat(&self) -> Mat4f {
        let trans = Mat4f::translation(Vector3f::new(
            -self.translation.x,
            -self.translation.y,
            -self.translation.z,
        ));

        let x_rot = Mat4f::rotation_x(-self.rotation.x);
        let y_rot = Mat4f::rotation_y(-self.rotation.y);
        let z_rot = Mat4f::rotation_z(-self.rotation.z);

        let rot = Mat4f::mult(&x_rot, &Mat4f::mult(&y_rot, &z_rot));

        let scale = Mat4f::scale(Vector3f::new(
            1f32/self.scale.x,
            1f32/self.scale.y,
            1f32/self.scale.z
        ));

        Mat4f::mult(&scale, &Mat4f::mult(&rot, &trans))
    }
}