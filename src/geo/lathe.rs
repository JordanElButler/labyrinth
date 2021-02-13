// create lathe geometry

use crate::math::{Vector2f};
use crate::mesh::Mesh;
use crate::vertex::{VertexLayout, AttributeType, Vertex};
pub fn gen_lathe_mesh(points: Vec<Vector2f>, segments: i32) -> Mesh {

    let width_components: usize = segments as usize;
    let height_components: usize = points.len() - 1;

    let num_vertices_hori: usize = width_components;
    let num_vertices_vert: usize = height_components + 1;

    let vertex_layout = VertexLayout::new(vec![AttributeType::Position3D, AttributeType::Normal3D, AttributeType::ST]);
    let mut vertices: Vec<f32> = Vec::with_capacity(vertex_layout.get_width() * num_vertices_hori as usize * num_vertices_vert as usize);
    let mut indices: Vec<i32> = Vec::with_capacity((2 * 3 * width_components * height_components) as usize);

    let rot_distance = num_vertices_hori;
    let height_distance = num_vertices_vert;

    // gen points
    for i in 0..num_vertices_vert {
        let point = points.get(i).unwrap();
        for j in 0..num_vertices_hori {
            let radians = j as f32 * 6.28318f32 / (num_vertices_hori as f32);
            vertices.push(point.x * radians.cos());
            vertices.push(point.y);
            vertices.push(point.x * radians.sin());

            vertices.push(0f32);
            vertices.push(0f32);
            vertices.push(1f32);

            vertices.push(j as f32 / rot_distance as f32);
            vertices.push(i as f32 / height_distance as f32);
        }
    }

    for y in 0..(height_components) {
        for x in 0..(width_components) {

            // as sheet unrolled
            let i1 = (x, y);
            let i2 = ((x+1) % num_vertices_hori, y);
            let i3 = (x, y+1);
            let i4 = ((x+1) % num_vertices_hori, y+1);

            let first_index = i1.1 * num_vertices_hori + i1.0;
            let second_index = i2.1 * num_vertices_hori + i2.0;
            let third_index =  i3.1 * num_vertices_hori + i3.0;
            let fourth_index =  i4.1 * num_vertices_hori + i4.0;

            indices.push(first_index as i32);
            indices.push(second_index as i32);
            indices.push(third_index as i32);

            indices.push(third_index as i32);
            indices.push(fourth_index as i32);
            indices.push(second_index as i32);
        }
    }
    let vertex = Vertex::new(vertices, indices, vertex_layout);
    Mesh::new(vertex)
}