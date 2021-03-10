/*
Partially derived from
https://github.com/Nercury/rust-and-opengl-lessons
*/
#[derive(Debug)]
pub enum AttributeType {
    Position2D,
    Position3D,
    Color,
    Normal3D,
    ST,
}

fn get_attribute_type_size(v: &AttributeType) -> usize {
    match v {
        AttributeType::Position2D => 2 * std::mem::size_of::<f32>(),
        AttributeType::Position3D => 3 * std::mem::size_of::<f32>(),
        AttributeType::Color => 3 * std::mem::size_of::<f32>(),
        AttributeType::Normal3D => 3 * std::mem::size_of::<f32>(),
        AttributeType::ST => 2 * std::mem::size_of::<f32>(),
    }
}
fn get_attribute_type_num_components(v: &AttributeType) -> usize {
    match v {
        AttributeType::Position2D => 2,
        AttributeType::Position3D => 3,
        AttributeType::Color => 3,
        AttributeType::Normal3D => 3,
        AttributeType::ST => 2,
    }
}

#[derive(Debug)]
pub struct VertexLayout {
    types: Vec<AttributeType>,
}

impl VertexLayout {
    pub fn new(vl: Vec<AttributeType>) -> Self {
        VertexLayout {
            types: vl,
        }
    }
    pub fn get_num_components(&self) -> usize {
        let mut components = 0;
        for vertex_type in self.types.iter() {
            components += get_attribute_type_num_components(vertex_type);
        }
        components
    }
    pub fn get_stride(&self) -> usize {
        let mut stride = 0;
        for vertex_type in self.types.iter() {
            stride += get_attribute_type_size(vertex_type);
        }
        stride
    }
    fn get_offset(&self, index: usize) -> usize {
        let count = self.types.len();
        if index >= count {
            panic!("get_offset: index exceeds len of VertexLayout");
        }
        let mut offset = 0;

        for i in 0..index {
            offset += get_attribute_type_size(self.types.get(i).unwrap());
        }
        offset
    }
    pub fn get_width(&self) -> usize {
        let mut width = 0;
        for vertex_type in self.types.iter() {
            width += get_attribute_type_size(vertex_type);
        }
        width
    }
}

#[derive(Debug)]
pub struct Vertex {
    pub vertex_layout: VertexLayout,
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub vertex_array: VertexArray,
    pub vertices: Vec<f32>,
    pub indices: Vec<i32>,
}

impl Vertex {
    pub fn new(vertices: Vec<f32>, indices: Vec<i32>, vertex_layout: VertexLayout) -> Self {
        let vertex_buffer = VertexBuffer::new();
        let index_buffer = IndexBuffer::new();
        let vertex_array = VertexArray::new();

        Vertex {
            vertex_layout,
            vertex_buffer,
            index_buffer,
            vertex_array,
            vertices,
            indices,
        }
    }
    pub fn buffer_data(&self) {
        self.vertex_buffer.buffer_data(&self.vertices);
        self.index_buffer.buffer_data(&self.indices);
    }
    pub fn set_attrib_pointers(&self) {
        self.vertex_buffer.bind();
        self.vertex_array.bind();
        for i in 0..self.vertex_layout.types.len() {
            let vertex_type = self.vertex_layout.types.get(i).unwrap();
            unsafe {
                gl::EnableVertexAttribArray(i as gl::types::GLuint);
                gl::VertexAttribPointer(
                    i as gl::types::GLuint,
                    get_attribute_type_num_components(&vertex_type) as gl::types::GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    self.vertex_layout.get_stride() as gl::types::GLint,
                    self.vertex_layout.get_offset(i) as *const gl::types::GLvoid
                );
            }
        }
        self.vertex_buffer.unbind();
        self.vertex_array.unbind();
    }
    pub fn get_num_vertices(&self) -> usize {
        self.vertices.len() / self.vertex_layout.get_num_components()
    }
    pub fn bind(&self) {
        self.vertex_array.bind();
        self.vertex_buffer.bind();
        self.index_buffer.bind();
    }
    pub fn unbind(&self) {
        self.vertex_array.unbind();
        self.vertex_buffer.unbind();
        self.index_buffer.unbind();
    }
    pub fn draw_call(&self) {
        self.vertex_array.bind();
        self.index_buffer.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as gl::types::GLsizei, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }
    }
    pub fn instanced_draw_call(&self, num: i32) {
        self.vertex_array.bind();
        self.index_buffer.bind();

        unsafe {
            gl::DrawElementsInstanced(gl::TRIANGLES, self.indices.len() as gl::types::GLsizei, gl:: UNSIGNED_INT, 0 as *const gl::types::GLvoid, num);
        }
    }
}

impl Drop for Vertex {
    fn drop(&mut self) {

    }
}

#[derive(Debug)]
pub struct VertexBuffer {
    vbo: gl::types::GLuint,
}

impl VertexBuffer {
    pub fn new() -> Self {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        VertexBuffer {
            vbo: vbo,
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
    pub fn buffer_data(&self, data: &Vec<f32>) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        self.unbind();
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

#[derive(Debug)]
pub struct IndexBuffer {
    ibo: gl::types::GLuint,
}

impl IndexBuffer {
    pub fn new() -> Self {
        let mut ibo = 0;
        unsafe {
            gl::GenBuffers(1, &mut ibo);
        }

        IndexBuffer {
            ibo: ibo,
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
    pub fn buffer_data(&self, data: &Vec<i32>) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }
        self.unbind();
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}

#[derive(Debug)]
pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            vao: vao,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}