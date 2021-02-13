/*
where all the materials go
dream: construct and compile needed shaders based on material property types at startup
*/

use crate::shader::Program;
use crate::resources::{ResourceKey, Resources};
use crate::math::Vector3f;

pub enum MaterialPropertyType<T> {
    PerVertex,
    Constant(T),
    FromTexture(ResourceKey),
}

pub struct Albedo( pub MaterialPropertyType<Vector3f> );

pub struct Normal( pub MaterialPropertyType<Vector3f> );

pub struct Metallic( pub MaterialPropertyType<f32> );

pub struct Roughness( pub MaterialPropertyType<f32> );

pub struct AO( pub MaterialPropertyType<f32> );

pub struct Material {
    pub albedo: Albedo,
    pub normal: Normal,
    pub metallic: Metallic,
    pub roughness: Roughness,
    pub ao: AO,
}

impl Material {
    pub fn make_basic_material(albedo: Vector3f, metallic: f32, roughness: f32, ao: f32) -> Self {
        Material {
            albedo: Albedo(MaterialPropertyType::Constant(albedo)),
            normal: Normal(MaterialPropertyType::<Vector3f>::PerVertex),
            metallic: Metallic(MaterialPropertyType::Constant(metallic)),
            roughness: Roughness(MaterialPropertyType::Constant(roughness)),
            ao: AO(MaterialPropertyType::Constant(ao)),
        }
    }
    pub fn load_shader_data(&self, res: &Resources, program: &Program) {
        match self.albedo.0 {
            MaterialPropertyType::Constant(v) => program.set3f("albedo", v.x, v.y, v.z).unwrap(),
            
            MaterialPropertyType::FromTexture(key) => {
                let tex = res.get_texture(key).unwrap();
                tex.load_memory();
                tex.bind();
                program.set1i("albedo", 0);
            },
            _ => panic!("Bad material")
        }
        match self.metallic.0 {
            MaterialPropertyType::Constant(f) => program.set1f("metallic", f).unwrap(),
            _ => panic!("Bad material")
        }
        match self.roughness.0 {
            MaterialPropertyType::Constant(f) => program.set1f("roughness", f).unwrap(),
            _ => panic!("Bad material")
        }
        match self.ao.0 {
            MaterialPropertyType::Constant(f) => program.set1f("ao", f).unwrap(),
            _ => panic!("Bad material")
        }
    }
}
