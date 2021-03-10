
use crate::{
    render_object::RenderObject,
    light::{Light, LightType},
    player::Player,
    camera::Camera,
    resources::Resources,
    material::Material,
    math::{Vector3f, Mat4f},
    transform::Transform,
};
pub struct Scene {
    pub objects: Vec::<RenderObject>,
    pub lights: Vec::<Light>
}

impl Scene {
    pub fn new() -> Self {
        pub fn create_initial_scene(res: &Resources) -> Scene {

            let mut scene = Scene::new();
    
            for x in 0..10 {
                for y in 0..10 {
                    let mut transform = Transform::identity();
                    transform.translation.x = (x - 5) as f32;
                    transform.translation.y = (y - 5) as f32;
                    transform.translation.z = 10 as f32;
        
                    transform.scale.x = 0.5;
                    transform.scale.y = 0.5;
                    transform.scale.z = 0.5;
        
                    scene.add_object( RenderObject::new(
                        res,
                        transform,
                        "g_program",
                        "sphere",
                        Material::make_basic_material(
                            Vector3f::new(1f32, 0f32, 0f32),
                            x as f32 / 10.0,
                            y as f32 / 10.0,
                            0.5f32)));
                }
            }
        
        
            let mut transform = Transform::identity();
            transform.translation.z = 5f32;
            scene.add_object(RenderObject::new(
                res,
                transform,
                "g_tex_program",
                "lab",
                Material {
                    albedo: Albedo(MaterialPropertyType::FromTexture(res.get_texture_id_by_name("dumb").unwrap())),
                    normal: Normal(MaterialPropertyType::PerVertex),
                    metallic: Metallic(MaterialPropertyType::Constant(1f32)),
                    roughness: Roughness(MaterialPropertyType::Constant(0f32)),
                    ao: AO(MaterialPropertyType::Constant(1f32)),
                }));
    
            for x in 0..2 {
                for y in 0..2 {
                    let mut transform = Transform::identity();
                    transform.translation.x = (5 * (x - 1)) as f32;
                    transform.translation.y = (5 * (y - 1))  as f32;
                    scene.add_light( Light::new_point_light(
                        transform,
                        Vector3f::new(1.0, 1.0, 1.0),
                        1.0
                    ));
                }
            }
    
            scene
        }
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
    pub fn update(&mut self, dt: i32) {

    }
    pub fn render(&mut self) {

    }
}