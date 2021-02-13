use crate::transform::Transform;
use crate::shader::{Program};
use crate::mesh::Mesh;
use crate::camera::PerspectiveCamera;
use crate::vertex::{Vertex};
use crate::resources::{ResourceKey, Resources};
use crate::light::Light;
use crate::material::{MaterialPropertyType, Material};

pub struct RenderObject {
    pub transform: Transform,
    pub program_key: ResourceKey,
    pub mesh_key: ResourceKey,
    pub material: Material,
    pub is_loaded: bool,
}

impl RenderObject {
    pub fn new(res: &mut Resources, transform: Transform, program_name: &str, mesh_name: &str, material: Material) -> Self {

        RenderObject {
            transform,
            program_key: res.get_program_id_by_name(program_name).unwrap(),
            mesh_key: res.get_mesh_id_by_name(mesh_name).unwrap(),
            material,
            is_loaded: false,
        }
    }
    pub fn draw(&mut self, res: &mut Resources, camera: &PerspectiveCamera) {
        self.material.load_shader_data(res, self.program_key, &self.transform, &camera);
        let mut mesh = res.get_mesh(self.mesh_key).unwrap();
        mesh.load();
        mesh.bind();
        mesh.draw();
        crate::gl_util::gl_dump_errors();

    }
}

pub struct Scene {
    pub objects: Vec::<RenderObject>,
    pub lights: Vec::<Light>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
    pub fn add_object(&mut self, obj: RenderObject) {
        self.objects.push(obj);
    }
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    pub fn get_objects(&self) -> &Vec<RenderObject> {
        &self.objects
    }
    pub fn get_lights(&self) -> &Vec<Light> {
        &self.lights
    }
}