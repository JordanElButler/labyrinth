use crate::vertex::{Vertex};

#[derive(Debug)]
pub struct Mesh {
    pub vertex: Vertex,
    pub is_loaded: bool,
}

impl Mesh {
    pub fn new(vertex: Vertex) -> Self {
        Mesh {
            vertex,
            is_loaded: false,
        }
    }
    pub fn load(&mut self) {
        self.vertex.buffer_data();
        self.vertex.set_attrib_pointers();
    }
    pub fn unload(&self) {

    }
    pub fn bind(&mut self) {
        self.vertex.bind();
    }
    pub fn unbind(&mut self) {
        self.vertex.unbind();
    }
    pub fn draw(&mut self) {
        self.vertex.draw_call();
    }
}