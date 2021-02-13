use crate::mesh::Mesh;
use crate::vertex::{Vertex, VertexLayout, AttributeType};

pub fn gen_screen_quad() -> Mesh {
    let quad_verts = vec![
        -1f32,  1f32, 0f32, 1f32,
         1f32,  1f32, 1f32, 1f32,
        -1f32, -1f32, 0f32, 0f32,
         1f32, -1f32, 1f32, 0f32,
    ];
    let quad_ind = vec![
        0, 1, 2,
        2, 3, 1,
    ];

    Mesh::new(Vertex::new(quad_verts, quad_ind, VertexLayout::new(vec![AttributeType::Position2D, AttributeType::ST])))
}