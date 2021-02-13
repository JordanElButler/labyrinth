
use crate::mesh::{Mesh};
use crate::math::Vector2f;
pub fn gen_sphere_mesh(width_components: i32, height_components: i32) -> Mesh {

    let num_vertices = (height_components + 1) as usize;
    let mut points: Vec<Vector2f> = Vec::with_capacity(num_vertices);
    for i in 0..num_vertices {
        // 270 + 180
        let radians =  i as f32 * 0.5 * 6.28318f32 / (height_components as f32) +  6.28318f32 * 3.0 / 4.0;
        let v = Vector2f::new(radians.cos(), radians.sin());
        points.push(v);
    }
    super::lathe::gen_lathe_mesh(points, width_components)
}