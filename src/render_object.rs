use crate::transform::Transform;
use crate::shader::{Program};
use crate::mesh::Mesh;
use crate::camera::PerspectiveCamera;
use crate::vertex::{Vertex};
use crate::resources::Resources;
use crate::light::PointLight;
use crate::material::{MaterialPropertyType, Material};

pub struct RenderObject<'a> {
    pub transform: Transform,
    pub program_id: &'a str,
    pub mesh_id: &'a str,
    pub material: Material,
    pub is_loaded: bool,
}

impl<'a> RenderObject<'a> {
    pub fn new(transform: Transform, program_id: &'a str, mesh_id: &'a str, material: Material) -> Self {

        RenderObject {
            transform,
            program_id,
            mesh_id,
            material,
            is_loaded: false,
        }
    }

    pub fn draw(&mut self, resources: &mut Resources, camera: &PerspectiveCamera) {
        let mut program = resources.get_program(self.program_id).unwrap();
        program.set_used();
        program.setMat4fv("proj", camera.projection_matrix.as_ptr()).unwrap();
        program.setMat4fv("view", camera.view_mat().as_ptr()).unwrap();
        program.setMat4fv("model", self.transform.model_mat().as_ptr()).unwrap();

        program.setMat4fv("model_rot", self.transform.model_rot().as_ptr()).unwrap();
        program.setMat4fv("view_rot", camera.view_rot().as_ptr()).unwrap();

        self.material.load_shader_data(&mut program);

        /*
        let mut texture = resources.get_texture(self.texture_id).unwrap();
        texture.load_memory();
        texture.bind();
        */

        let mut mesh = resources.get_mesh(self.mesh_id).unwrap();
        mesh.load();
        mesh.bind();
        
        mesh.draw();
        crate::gl_util::gl_dump_errors();

    }
}
/*
pub struct Scene {
    objects: Vec::<RenderObject>,
    lights: Vec::<Light>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new()
        }
    }
    pub fn add_object(&mut self, obj: RenderObject) {
        self.objects.push(obj);
    }
    pub fn 
}

*/