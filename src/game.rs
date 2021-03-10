use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{GLProfile, GLContext, Window},
    EventPump,
};
use std::{
    time::Duration,
};

use crate::{
    timer::Timer,
    input::InputState,
    render_object::{TerrainChunkObject, RenderObject},
    resources::Resources,
    transform::{Basis, Transform},
    light::Light,
    camera::Camera,
    material::*,
    math::*,
    renderer::Renderer,
    managers::Manager,
};

pub struct Player {
    transform: Transform,
    basis: Basis,
    velocity: Vector3f,
    gun: RenderObject,
}
impl Player {
    pub fn new(res: &Resources) -> Self {
        Player {
            transform: Transform::identity(),
            basis: Basis::new(Vector3f::new(1f32, 0f32, 0f32), Vector3f::new(0f32, 1f32, 0f32), Vector3f::new(0f32, 0f32, -1f32)),
            velocity: Vector3f::zero(),
            gun: RenderObject::new(
                res,
                Transform::identity(),
                "g_program",
                "gun",
                Material::make_basic_material(
                    Vector3f::new(0.8, 0.8, 0.8), 
                    0.9, 
                    0.01, 
                    0.1),
            )
        }
    }
    pub fn update(&mut self, manager: &Manager, dt: i32) {

        // movement stuff
        let new_transformed_basis = self.basis.transform_basis(&self.transform);
        let input = manager.get_input();
        let mut scale_vector = Vector3f::zero();
        if input.key_state.key_down(&Keycode::A) {
            scale_vector.x -= 1f32;
        } else if input.key_state.key_down(&Keycode::D) {
            scale_vector.x += 1f32;
        }
        
        if input.key_state.key_down(&Keycode::W) {
            scale_vector.z += 1f32;
        } else if input.key_state.key_down(&Keycode::S) {
            scale_vector.z -= 1f32;
        }
        if input.key_state.key_down(&Keycode::R) {
            scale_vector.y += 1f32;
        } else if input.key_state.key_down(&Keycode::F) {
            scale_vector.y -= 1f32;
        }
        if !scale_vector.is_zero() {
            scale_vector.scale(0.01 * dt as f32);
        }

        let move_vector = new_transformed_basis.scale_and_add(&scale_vector);
        self.velocity.add_to(&move_vector);


        if input.mouse_state.movement() {
            let mut mv = input.mouse_state.get_direction_normal();

            self.transform.rotation.y -= mv.x * 0.01f32;
            self.transform.rotation.x -= mv.y * 0.01f32;
            if self.transform.rotation.x > std::f32::consts::FRAC_PI_2 {
                self.transform.rotation.x = std::f32::consts::FRAC_PI_2;
            } else if self.transform.rotation.x < -std::f32::consts::FRAC_PI_2 {
                self.transform.rotation.x = -std::f32::consts::FRAC_PI_2
            }
        }

        self.transform.translation.add_to(&self.velocity);
        self.velocity.scalar(0.7);

        // gun
        let new_transformed_basis = self.basis.transform_basis(&self.transform);
        self.gun.transform = self.transform;
        self.gun.transform.translation.add_to(&new_transformed_basis.scale_and_add(&Vector3f::new(0.4f32, -0.5f32, 0.5f32)));
    }
    pub fn draw(&self, res: &Resources, camera: &Camera) {
        self.gun.draw(res, camera);
    }
    pub fn front(&self) -> Vector3f {
        self.basis.transform_basis(&self.transform).v3
    }
}
pub struct Scene {
    pub targets: Vec::<Target>,
    pub objects: Vec::<RenderObject>,
    pub terrain_chunks: Vec::<TerrainChunkObject>,
    pub lights: Vec::<Light>,
    pub camera: Camera,
    pub player: Player,
}

impl Scene {
    pub fn new(res: &Resources, camera: Camera) -> Self {
        let mut scene = Scene {
            targets: Vec::new(),
            objects: Vec::new(),
            terrain_chunks: Vec::new(),
            lights: Vec::new(),
            camera,
            player: Player::new(res),
        };
        scene.init_scene(res);
        scene
    }
    pub fn init_scene(&mut self, res: &Resources) {

        for x in 0..1 {
            for z in 0..1 {
                let mut transform = Transform::identity();
                transform.translation.x = (x - 50) as f32;
                transform.translation.z = (z - 50) as f32;
                transform.translation.y = -7f32;

                self.terrain_chunks.push(TerrainChunkObject::new(
                    res,
                    transform,
                    "g_terrain_program",
                    "cube",
                    Material::make_basic_material(
                        Vector3f::new(0f32, 0.4f32, 0f32),
                        0.3f32,
                        0.8f32,
                        0.2f32
                    )
                ))
            }
        }

        for x in 0..10 {
            for y in 0..10 {
                let mut transform = Transform::identity();
                transform.translation.x = (x - 5) as f32;
                transform.translation.y = (y - 5) as f32;
                transform.translation.z = -5f32;
    
                transform.scale.x = 0.5;
                transform.scale.y = 0.5;
                transform.scale.z = 0.5;
    
                self.add_target( Target::new(
                    res,
                    transform,
                    Material::make_basic_material(
                        Vector3f::new(1f32, 0f32, 0f32),
                        x as f32 / 10.0,
                        y as f32 / 10.0,
                        0.5f32)));
            }
        }
    
    
        let mut transform = Transform::identity();
        transform.translation.z = -5f32;
        self.add_object(RenderObject::new(
            res,
            transform,
            "g_tex_program",
            "lab",
            Material {
                albedo: Albedo(MaterialPropertyType::FromTexture(res.get_texture_id_by_name("dumb").unwrap())),
                normal: Normal(MaterialPropertyType::PerVertex),
                metallic: Metallic(MaterialPropertyType::Constant(1f32)),
                roughness: Roughness(MaterialPropertyType::Constant(0.2f32)),
                ao: AO(MaterialPropertyType::Constant(1f32)),
            }));

        for x in 0..2 {
            for y in 0..2 {
                let mut transform = Transform::identity();
                transform.translation.x = (5 * (x - 1)) as f32;
                transform.translation.y = (5 * (y - 1))  as f32;
                transform.translation.z = 1.0f32;
                transform.scale.x = 0.1;
                transform.scale.y = 0.1;
                transform.scale.z = 0.1;
                self.add_light( Light::new_point_light(
                    transform,
                    Vector3f::new(1.0, 1.0, 1.0),
                    1.0
                ));
            }
        }
    }
    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
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
    pub fn update(&mut self, manager: &Manager, dt: i32) {


        self.player.update(manager, dt);
        self.camera.transform = self.player.transform;

        let input = manager.get_input();
        if (input.mouse_state.left) {
            let ray = Ray{
                origin: self.player.transform.translation, 
                direction: self.player.front(),
            };
            for target in self.targets.iter_mut() {
                if target.ray_intersection(&ray) {
                    target.set_hit();
                }
            }
        }

        for obj in self.objects.iter_mut() {
            obj.transform.rotation.y += 0.001 * dt as f32;
        }
        for target in self.targets.iter_mut() {
            target.update(dt);
        }

        // move the lights around
        // one rotation every second -- dt == 1000
        for i in 0..self.lights.len() {
            let mut light = self.lights.get_mut(i).unwrap();
            light.transform.rotation.x += dt as f32;
            light.transform.translation.x = 4f32 * ((light.transform.rotation.x / 1000f32 + (i as f32) * std::f32::consts::FRAC_PI_2)).cos();
            light.transform.translation.y = 4f32 * ((light.transform.rotation.x / 1000f32 + (i as f32) * std::f32::consts::FRAC_PI_2)).sin();
        }
    }
}
pub struct Ray {
    origin: Vector3f,
    direction: Vector3f,
}
pub struct Target {
    sphere_render_object: RenderObject,
    dt: f32,
}
impl Target {
    pub fn new(res: &Resources, transform: Transform, material: Material) -> Self {
        Target {
            sphere_render_object: RenderObject::new(
                res,
                transform,
                "g_program",
                "sphere",
                material),
                dt: 0f32,
            }
    }
    pub fn update(&mut self, dt: i32) {
        self.dt -= (dt as f32) / 1000f32;
        if self.dt < 0f32 {
            self.dt = 0f32;
        }
        self.sphere_render_object.material.albedo = Albedo(MaterialPropertyType::Constant(Vector3f::interpolate(&Vector3f::new(1f32, 0f32, 0f32), &Vector3f::new(0f32, 1f32, 0f32), self.dt)));
    }
    pub fn ray_intersection(&self, ray: &Ray) -> bool {
        let so = self.sphere_render_object.transform.translation;
        let sr = 0.5f32; // really very bad trial and error guessing what the radius of the scaled sphere meshes is without even taking into account the possible scale factor from constructor

        let diffo = Vector3f::sub(&ray.origin, &so);

        let p = Vector3f::dot(&ray.direction, &diffo);
        let q = Vector3f::dot(&diffo, &diffo) - sr * sr;

        let disc = p * p - q;
        if disc < 0f32 {
            return false;
        } else {
            return true;
        }
    }
    pub fn set_hit(&mut self) {
        self.dt = 1f32;
    }
    pub fn draw(&self, res: &Resources, camera: &Camera) {
        self.sphere_render_object.draw(res, camera);
    }
}
pub struct GameState {
    scene: Scene,
}

impl GameState {
    pub fn new(res: &Resources, window_width: u32, window_height: u32) -> Self {

        let mut camera = Camera::new_perspective_camera(80.0, (window_width as f32) / (window_height as f32), 0.01f32, 1000f32);

        GameState {
            scene: Scene::new(res, camera),
        }
    }
    pub fn update(&mut self, manager: &Manager, dt: i32) {
        self.scene.update(&manager, dt);
    }
    pub fn render(&self, manager: &Manager, renderer: &Renderer) {
        renderer.render(manager.get_res(), &self.scene);
    }
}

pub struct GameApp {
    game_state: GameState,
    sdl_context: sdl2::Sdl,
    window: Window,
    window_width: i32,
    window_height: i32,
    gl_context: GLContext,
    event_pump: EventPump,
    manager: Manager,
    renderer: Renderer,
    sdl_mouse_util: sdl2::mouse::MouseUtil,
}

impl GameApp {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3,3);
    
        let window_height = 900;
        let window_width = 1200;
        let window = video_subsystem.window("gl_fun", window_width as u32, window_height as u32)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();
    

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
        let sdl_mouse_util = sdl_context.mouse();
        sdl_mouse_util.show_cursor(false);
        sdl_mouse_util.set_relative_mouse_mode(true);
        let mut event_pump = sdl_context.event_pump().unwrap();
    
        let manager = Manager::new();

        let renderer = Renderer::new(window_width, window_height);

        GameApp {
            game_state: GameState::new(manager.get_res(), window_width, window_height),
            sdl_context,
            window,
            window_width: window_width as i32,
            window_height: window_height as i32,
            gl_context,
            event_pump,
            manager,
            renderer,
            sdl_mouse_util
        }
    }
    pub fn game_loop(&mut self) {
        self.manager.timer.start();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {
                        self.manager.input.update(&event);
                    }
                }
            }
            self.update();
            self.render();
            crate::gl_util::gl_dump_errors();
            self.window.gl_swap_window();
            // dirty
            if !(self.sdl_mouse_util.focused_window_id() == None) {
                self.sdl_context.mouse().warp_mouse_in_window(&self.window, self.window_width/2, self.window_height/2);
                //self.manager.input.mouse_state.x = self.window_width as f32/2f32;
                //self.manager.input.mouse_state.y = self.window_height as f32/2f32;
            }
        }
    }
    pub fn start(&mut self) {
        self.game_loop();
    }
    pub fn update(&mut self) {
        self.manager.timer.update();
        self.game_state.update(&self.manager, self.manager.timer.get_elapsed_ms());
    }
    pub fn render(&self) {
        self.game_state.render(&self.manager, &self.renderer);
    }
}