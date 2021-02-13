use crate::math::{Vector3f, Mat4f};

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
        let mut z_rot = Mat4f::identity();
        z_rot.set(0, 0, self.rotation.z.cos());
        z_rot.set(0, 1, self.rotation.z.sin());
        z_rot.set(1, 0, -self.rotation.z.sin());
        z_rot.set(1, 1, self.rotation.z.cos());

        let mut y_rot = Mat4f::identity();
        y_rot.set(0, 0, self.rotation.y.cos());
        y_rot.set(0, 2, self.rotation.y.sin());
        y_rot.set(2, 0, -self.rotation.y.sin());
        y_rot.set(2, 2, self.rotation.y.cos());

        let mut x_rot = Mat4f::identity();
        x_rot.set(1, 1, self.rotation.x.cos());
        x_rot.set(1, 2, self.rotation.x.sin());
        x_rot.set(2, 1, -self.rotation.x.sin());
        x_rot.set(2, 2, self.rotation.x.cos());

        Mat4f::mult(&z_rot, &Mat4f::mult(&y_rot, &x_rot))
    }
    pub fn model_mat(&self) -> Mat4f {
        let mut scale = Mat4f::identity();
        scale.set(0, 0, self.scale.x);
        scale.set(1, 1, self.scale.y);
        scale.set(2, 2, self.scale.z);

        let mut z_rot = Mat4f::identity();
        z_rot.set(0, 0, self.rotation.z.cos());
        z_rot.set(0, 1, self.rotation.z.sin());
        z_rot.set(1, 0, -self.rotation.z.sin());
        z_rot.set(1, 1, self.rotation.z.cos());

        let mut y_rot = Mat4f::identity();
        y_rot.set(0, 0, self.rotation.y.cos());
        y_rot.set(0, 2, self.rotation.y.sin());
        y_rot.set(2, 0, -self.rotation.y.sin());
        y_rot.set(2, 2, self.rotation.y.cos());

        let mut x_rot = Mat4f::identity();
        x_rot.set(1, 1, self.rotation.x.cos());
        x_rot.set(1, 2, self.rotation.x.sin());
        x_rot.set(2, 1, -self.rotation.x.sin());
        x_rot.set(2, 2, self.rotation.x.cos());

        let rot = Mat4f::mult(&z_rot, &Mat4f::mult(&y_rot, &x_rot));

        let mut trans = Mat4f::identity();
        trans.set(3, 0, self.translation.x);
        trans.set(3, 1, self.translation.y);
        trans.set(3, 2, self.translation.z);
        
        Mat4f::mult(&trans, &Mat4f::mult(&rot, &scale))

    }

    // inverse of transform matrix
    pub fn inverse_mat(&self) -> Mat4f {
        let mut trans = Mat4f::identity();
        trans.set(3, 0, -self.translation.x);
        trans.set(3, 1, -self.translation.y);
        trans.set(3, 2, -self.translation.z);

        let mut x_rot = Mat4f::identity();
        x_rot.set(1, 1, self.rotation.x.cos());
        x_rot.set(2, 1, self.rotation.x.sin());
        x_rot.set(1, 2, -self.rotation.x.sin());
        x_rot.set(2, 2, self.rotation.x.cos());

        let mut y_rot = Mat4f::identity();
        y_rot.set(0, 0, self.rotation.y.cos());
        y_rot.set(2, 0, self.rotation.y.sin());
        y_rot.set(0, 2, -self.rotation.y.sin());
        y_rot.set(2, 2, self.rotation.y.cos());

        let mut z_rot = Mat4f::identity();
        z_rot.set(0, 0, self.rotation.z.cos());
        z_rot.set(1, 0, self.rotation.z.sin());
        z_rot.set(0, 1, -self.rotation.z.sin());
        z_rot.set(1, 1, self.rotation.z.cos());

        let rot = Mat4f::mult(&x_rot, &Mat4f::mult(&y_rot, &z_rot));

        let mut scale = Mat4f::identity();
        scale.set(0, 0, 1f32/self.scale.x);
        scale.set(1, 1, 1f32/self.scale.y);
        scale.set(2, 2, 1f32/self.scale.z);

        Mat4f::mult(&scale, &Mat4f::mult(&rot, &trans))
    }
}