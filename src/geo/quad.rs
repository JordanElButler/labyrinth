/*
    Quad mesh
*/
use crate::mesh::{Mesh};
use crate::vertex::{Vertex, VertexLayout, AttributeType};
pub fn gen_quad_mesh(width_components: i32, height_components: i32) -> Mesh {

    let num_vertices_hori = width_components + 1;
    let num_vertices_vert = height_components + 1;

    let vertex_layout = VertexLayout::new(vec![AttributeType::Position3D, AttributeType::Normal3D, AttributeType::ST]);
    let mut vertices: Vec<f32> = Vec::with_capacity(vertex_layout.get_width() * num_vertices_hori as usize * num_vertices_vert as usize);
    let mut indices: Vec<i32> = Vec::with_capacity((2 * 3 * width_components * height_components) as usize);
    
    let x_stride = 1f32 / width_components as f32;
    let y_stride = 1f32 / height_components as f32;

    for y in 0..num_vertices_vert {
        for x in 0..num_vertices_hori {

            vertices.push(x as f32 * x_stride);
            vertices.push(y as f32 * y_stride);
            vertices.push(0f32);

            vertices.push(0f32);
            vertices.push(0f32);
            vertices.push(1f32);

            vertices.push(x as f32 * x_stride);
            vertices.push(y as f32 * y_stride);

        }
    }
    
    for y in 0..(height_components) {
        for x in 0..(width_components) {
            let first_index = y * num_vertices_hori + x;
            let second_index = first_index + 1;
            let third_index = first_index + num_vertices_hori;
            let fourth_index = second_index + num_vertices_hori;
            indices.push(first_index);
            indices.push(second_index);
            indices.push(third_index);

            indices.push(third_index);
            indices.push(fourth_index);
            indices.push(second_index);
        }
    }
    let vertex = Vertex::new(vertices, indices, vertex_layout);
    Mesh::new(vertex)
}