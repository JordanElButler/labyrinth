use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::GLProfile;
use sdl2::video::GLContext;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::timer::Timer;
use crate::input::InputState;
use crate::render_object::RenderObject;
use crate::resources::Resources;
use crate::transform::Transform;
use crate::light::Light;
use crate::render_object::Scene;
use crate::camera::Camera;
use crate::material::*;
use crate::math::*;
use crate::renderer::Renderer;
use crate::managers::Manager;

pub struct GameState {
    scene: Scene,
    camera: Camera,
    fixed_time_slice: i32,
    time_fragment: i32,
}

impl GameState {
    pub fn new(res: &Resources, window_width: u32, window_height: u32) -> Self {

        let mut camera = Camera::new_perspective_camera(80f32, (window_width as f32) / (window_height as f32), 0.01f32, 100f32);

        GameState {
            scene: GameState::create_initial_scene(res),
            camera,
            fixed_time_slice: 15i32,
            time_fragment: 0i32
        }
    }
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
    pub fn update(&mut self, manager: &Manager, dt: i32) {
        self.time_fragment += dt;
        while self.time_fragment >= self.fixed_time_slice {
            self.fixed_update(&manager);
            self.time_fragment -= self.fixed_time_slice;
        }
    }
    pub fn fixed_update(&mut self, manager: &Manager) {
        //self.scene.update(&manager, fixed_time_slice);
    
        let mut move_vector = Vector3f::new(0f32, 0f32, 0f32);
        let input = manager.get_input();
        if input.key_state.key_down(&Keycode::A) {
            move_vector.x -= 1f32;
        } else if input.key_state.key_down(&Keycode::D) {
            move_vector.x += 1f32;
        }
        
        if input.key_state.key_down(&Keycode::W) {
            move_vector.z += 1f32;
        } else if input.key_state.key_down(&Keycode::S) {
            move_vector.z -= 1f32;
        }
        if input.key_state.key_down(&Keycode::R) {
            move_vector.y += 1f32;
        } else if input.key_state.key_down(&Keycode::F) {
            move_vector.y -= 1f32;
        }
        if !move_vector.is_zero() {
            move_vector.scale(0.01 * self.fixed_time_slice as f32);
        }
        self.camera.transform.translation.add_to(&move_vector);

        if input.mouse_state.left {
            println!("{:?}\n{}", input.mouse_state, input.mouse_state.movement());
        }
        if input.mouse_state.right {
            println!("right mouse button down");
        }
        if input.mouse_state.movement() {
            let mv = input.mouse_state.get_direction_normal();
            self.camera.transform.rotation.y -= mv.x * 0.005 * self.fixed_time_slice as f32;
            self.camera.transform.rotation.x += mv.y * 0.005 * self.fixed_time_slice as f32;
            if self.camera.transform.rotation.x > std::f32::consts::FRAC_PI_2 {
                self.camera.transform.rotation.x = std::f32::consts::FRAC_PI_2;
            } else if self.camera.transform.rotation.x < -std::f32::consts::FRAC_PI_2 {
                self.camera.transform.rotation.x = -std::f32::consts::FRAC_PI_2
            }
        }
    }
    pub fn render(&self, manager: &Manager, renderer: &Renderer) {
        renderer.render(manager.get_res(), &self.scene, &self.camera);
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
    renderer: Renderer
}

impl GameApp {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3,3);
    
        let window_height = 600;
        let window_width = 600;
        let window = video_subsystem.window("gl_fun", window_width as u32, window_height as u32)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();
    

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
        sdl_context.mouse().show_cursor(false);
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

            self.manager.timer.update();
            self.update();
            self.render();
            crate::gl_util::gl_dump_errors();
            self.window.gl_swap_window();
            self.sdl_context.mouse().warp_mouse_in_window(&self.window, self.window_width/2, self.window_height/2);
            self.manager.input.mouse_state.x = self.window_width as f32/2f32;
            self.manager.input.mouse_state.y = self.window_height as f32/2f32;
        }
    }
    pub fn start(&mut self) {
        self.game_loop();
    }
    pub fn update(&mut self) {
        self.game_state.update(&self.manager, self.manager.timer.get_elapsed_ms());
    }
    pub fn render(&self) {
        self.game_state.render(&self.manager, &self.renderer);
    }
}