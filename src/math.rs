
#[derive(Debug, Copy, Clone)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f {
            x: x,
            y: y,
        }
    }
    pub fn zero() -> Vector2f {
        Vector2f::new(0.0, 0.0)
    }
    pub fn is_zero(&self) -> bool {
        self.x == 0f32 && self.y == 0f32
    }
    pub fn copy(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }
    pub fn mag_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    pub fn mag(&self) -> f32 {
        (self.mag_squared() as f64).sqrt() as f32
    }
    pub fn add_to(&mut self, v: &Vector2f) {
        self.x += v.x;
        self.y += v.y;
    }
    pub fn scalar(&mut self, r: f32) {
        self.x *= r;
        self.y *= r;
    }
    pub fn normalize(&mut self) {
        let mag = self.mag();
        if mag == 0f32 {
            panic!("Attempt to normalize 0 vector!!");
        }
        self.scalar(1.0 / mag);
    }
    pub fn scale(&mut self, r: f32) {
        self.normalize();
        self.scalar(r);
    }
    pub fn dot(v1: Vector2f, v2: Vector2f) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }
    pub fn cross(v1: Vector2f, v2: Vector2f, v3: Vector2f) -> f32 {
        (v2.x - v1.x)*(v3.y - v1.y) - (v2.y - v1.y)*(v3.x - v1.x)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f {
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn zero() -> Vector3f {
        Vector3f::new(0.0, 0.0, 0.0)
    }
    pub fn is_zero(&self) -> bool {
        self.x == 0f32 && self.y == 0f32 && self.z == 0f32
    }
    pub fn copy(&self) -> Vector3f {
        Vector3f::new(self.x, self.y, self.z)
    }
    pub fn mag_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn mag(&self) -> f32 {
        (self.mag_squared() as f64).sqrt() as f32
    }
    pub fn add(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
        Vector3f::new(
            v1.x + v2.x,
            v1.y + v2.y,
            v1.z + v2.z)
    }
    pub fn add_to(&mut self, v: &Vector3f) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
    pub fn sub(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
        Vector3f::new(
            v1.x - v2.x,
            v1.y - v2.y,
            v1.z - v2.z)
    }
    pub fn scalar(&mut self, r: f32) {
        self.x *= r;
        self.y *= r;
        self.z *= r;
    }
    pub fn normalize(&mut self) {
        let mag = self.mag();
        if mag == 0f32 {
            panic!("Attempt to normalize 0 vector!!");
        }
        self.scalar(1.0 / mag);
    }
    pub fn scale(&mut self, r: f32) {
        self.normalize();
        self.scalar(r);
    }
    pub fn dot(v1: &Vector3f, v2: &Vector3f) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }
    pub fn cross(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
        Vector3f {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
    pub fn interpolate(v1: &Vector3f, v2: &Vector3f, dt: f32) -> Vector3f {
        Vector3f {
            x: v1.x + (v2.x - v1.x) * dt,
            y: v1.y + (v2.y - v1.y) * dt,
            z: v1.z + (v2.z - v1.z) * dt,
        }
    }
}

pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4f {
        Vector4f {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn zero() -> Vector4f {
        Vector4f::new(0.0, 0.0, 0.0, 0.0)
    }
    pub fn copy(&self) -> Vector4f {
        Vector4f::new(self.x, self.y, self.z, self.w)
    }
    pub fn mag_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    pub fn mag(&self) -> f32 {
        (self.mag_squared() as f64).sqrt() as f32
    }
    pub fn add_to(&mut self, v: &Vector4f) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self.w += v.w;
    }
    pub fn scalar(&mut self, r: f32) {
        self.x *= r;
        self.y *= r;
        self.z *= r;
        self.w *= r;
    }
    pub fn normalize(&mut self) {
        let mag = self.mag();
        if mag == 0f32 {
            panic!("Attempt to normalize 0 vector!!");
        }
        self.scalar(1.0 / mag);
    }
    pub fn scale(&mut self, r: f32) {
        self.normalize();
        self.scalar(r);
    }
    pub fn dot(v1: Vector4f, v2: Vector4f) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
    }
}

#[derive(Debug)]
pub struct Mat4f {
    pub entries: Vec<f32>,
}


impl Mat4f {
    pub fn get(&self, column: usize, row: usize) -> f32 {
        self.entries[row * 4 + column]
    }
    pub fn set(&mut self, column: usize, row: usize, val: f32) {
        self.entries[row * 4 + column] = val;
    }
    pub fn identity() -> Mat4f {
        let mut entries = Vec::<f32>::with_capacity(16);
        for y in 0..4 {
            for x in 0..4 {
                if x == y {
                    entries.push(1.0f32);
                } else {
                    entries.push(0.0f32);
                }
            }
        }
        Mat4f {
            entries: entries
        }
    }
    pub fn copy(&self) -> Mat4f {
        let mut mat_copy = Mat4f::identity();
        mat_copy.entries.copy_from_slice(&self.entries[..]);
        mat_copy
    }
    pub fn scalar(&mut self, r: f32) {
        for entry in self.entries.iter_mut() {
            *entry *= r;
        }
    }
    pub fn mult(m1: &Mat4f, m2: &Mat4f) -> Mat4f {
        let mut prod_matrix = Mat4f::identity();
        for y in 0..4 {
            for x in 0..4 {
                let mut prod = 0f32;
                for k in 0..4 {
                    prod += m1.get(x, k) * m2.get(k, y);
                }
                prod_matrix.set(x, y, prod);
            }
        }
        prod_matrix
    }
    pub fn as_ptr(&self) -> *const f32 {
        self.entries.as_ptr()
    }
    pub fn rotation_x(rad: f32) -> Mat4f {
        let s = rad.sin();
        let c = rad.cos();
        let mut x_rot = Mat4f::identity();
        x_rot.set(1, 1, c);
        x_rot.set(2, 1, s);
        x_rot.set(1, 2, -s);
        x_rot.set(2, 2, c);
        x_rot
    }
    pub fn rotation_y(rad: f32) -> Mat4f {
        let s = rad.sin();
        let c = rad.cos();
        let mut y_rot = Mat4f::identity();
        y_rot.set(0, 0, c);
        y_rot.set(2, 0, -s);
        y_rot.set(0, 2, s);
        y_rot.set(2, 2, c);
        y_rot
    }
    pub fn rotation_z(rad: f32) -> Mat4f {
        let s = rad.sin();
        let c = rad.cos();
        let mut z_rot = Mat4f::identity();
        z_rot.set(0, 0, c);
        z_rot.set(1, 0, s);
        z_rot.set(0, 1, -s);
        z_rot.set(1, 1, c);
        z_rot
    }
    pub fn translation(t: Vector3f) -> Mat4f {
        let mut trans = Mat4f::identity();
        trans.set(0, 3, t.x);
        trans.set(1, 3, t.y);
        trans.set(2, 3, t.z);
        trans
    }
    pub fn scale(s: Vector3f) -> Mat4f {
        let mut scale = Mat4f::identity();
        scale.set(0, 0, s.x);
        scale.set(1, 1, s.y);
        scale.set(2, 2, s.z);
        scale
    }
    pub fn lookAt(position: &Vector3f, target: &Vector3f, up: &Vector3f) -> Mat4f {
        let mut z_axis = {
            let pt = position.copy();
            let mut tt = target.copy();
            tt.scale(-1f32);

            let mut zv = Vector3f::add(&pt, &tt);
            zv.normalize();
            zv
        };
        let x_axis = {
            let mut xc = Vector3f::cross(up, &z_axis);
            xc.normalize();
            xc
        };
        let y_axis = {
            let mut yc = Vector3f::cross(&z_axis, &x_axis);
            yc.normalize();
            yc
        };
        let mut mat = Mat4f::identity();
        mat.set(0, 0, x_axis.x);
        mat.set(0, 1, x_axis.y);
        mat.set(0, 2, x_axis.z);
        mat.set(1, 0, y_axis.x);
        mat.set(1, 1, y_axis.y);
        mat.set(1, 2, y_axis.z);
        mat.set(2, 0, z_axis.x);
        mat.set(2, 1, z_axis.y);
        mat.set(2, 2, z_axis.z);
        mat.set(3, 0, position.x);
        mat.set(3, 1, position.y);
        mat.set(3, 2, position.z);
        mat
    }
}
