use crate::transform::Transform;
use crate::shader::{Program};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::vertex::{Vertex};
use crate::resources::{ResourceKey, Resources};
use crate::light::Light;
use crate::material::{MaterialPropertyType, Material};

pub struct RenderObject {
    pub transform: Transform,
    pub program_key: ResourceKey,
    pub mesh_key: ResourceKey,
    pub material: Material,
}

impl RenderObject {
    pub fn new(res: &Resources, transform: Transform, program_name: &str, mesh_name: &str, material: Material) -> Self {

        RenderObject {
            transform,
            program_key: res.get_program_id_by_name(program_name).unwrap(),
            mesh_key: res.get_mesh_id_by_name(mesh_name).unwrap(),
            material,
        }
    }
    pub fn draw(&self, res: &Resources, camera: &Camera) {
        let program = res.get_program(self.program_key).unwrap();
        program.set_used();
        program.setMat4fv("proj", camera.proj_mat().as_ptr()).unwrap();
        program.setMat4fv("view", camera.view_mat().as_ptr()).unwrap();
        program.setMat4fv("model", self.transform.model_mat().as_ptr()).unwrap();

        program.setMat4fv("model_rot", self.transform.model_rot().as_ptr()).unwrap();
        self.material.load_shader_data(res, &program);
        let mesh = res.get_mesh(self.mesh_key).unwrap();
        mesh.load();
        mesh.bind();
        mesh.draw();
        crate::gl_util::gl_dump_errors();

    }
}

// dumb experiment with instancing
pub struct TerrainChunkObject {
    pub transform: Transform,
    pub program_key: ResourceKey,
    pub mesh_key: ResourceKey,
    pub material: Material,
}

impl TerrainChunkObject {
    pub fn new(res: &Resources, transform: Transform, program_name: &str, mesh_name: &str, material: Material) -> Self {

        TerrainChunkObject {
            transform,
            program_key: res.get_program_id_by_name(program_name).unwrap(),
            mesh_key: res.get_mesh_id_by_name(mesh_name).unwrap(),
            material,
        }
    }
    pub fn draw(&self, res: &Resources, camera: &Camera) {
        let program = res.get_program(self.program_key).unwrap();
        program.set_used();
        program.setMat4fv("proj", camera.proj_mat().as_ptr()).unwrap();
        program.setMat4fv("view", camera.view_mat().as_ptr()).unwrap();
        program.setMat4fv("model", self.transform.model_mat().as_ptr()).unwrap();

        program.setMat4fv("model_rot", self.transform.model_rot().as_ptr()).unwrap();
        program.setMat4fv("view_rot", camera.view_rot().as_ptr()).unwrap();

        let numX = 100;
        let numZ = 500;
        program.set1i("numX", numX);
        program.set1i("numZ", numZ);
        self.material.load_shader_data(res, &program);
        let mesh = res.get_mesh(self.mesh_key).unwrap();
        mesh.load();
        mesh.bind();
        mesh.instanced_draw(numX * numZ);
        crate::gl_util::gl_dump_errors();

    }
}