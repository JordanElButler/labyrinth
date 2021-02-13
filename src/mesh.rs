use crate::vertex::{Vertex};

#[derive(Debug)]
pub struct Mesh {
    pub vertex: Vertex,
}

impl Mesh {
    pub fn new(vertex: Vertex) -> Self {
        Mesh {
            vertex,
        }
    }
    pub fn load(&self) {
        self.vertex.buffer_data();
        self.vertex.set_attrib_pointers();
    }
    pub fn unload(&self) {

    }
    pub fn bind(&self) {
        self.vertex.bind();
    }
    pub fn unbind(&self) {
        self.vertex.unbind();
    }
    pub fn draw(&self) {
        self.vertex.draw_call();
    }
}