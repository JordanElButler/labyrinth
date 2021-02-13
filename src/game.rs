
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Texture;
use sdl2::video::GLProfile;

use crate::timer::Timer;
use crate::input::InputState;
use crate::render_object::RenderObject;
use crate::resources::Resources;
use crate::transform::Transform;
use crate::light::PointLight;

pub struct GameState {
    objs: Vec::<RenderObject>,
    quad_mesh: Mesh,
    lights: Vec::<PointLight>,
}

impl GameState {
    pub fn new() -> Self {
        let mut objs = Vec::<RenderObject>::new();

        let mut transform = Transform::identity();
        transform.translation.z = 5f32;
        let mut first_obj = RenderObject::new(
            transform,
            "textured_program",
            "face",
            "face");
    
    
        let mut transform = Transform::identity();
        transform.translation.x = 3f32;
        let mut second_obj = RenderObject::new(
            transform,
            "phong_program",
            "lab",
            "dumb");
    
        transform.translation.x = -3f32;
        let mut third_obj = RenderObject::new(
            transform,
            "textured_program",
            "sphere",
            "dumb");

        objs.push(first_obj);
        objs.push(second_obj);
        objs.push(third_obj);

        let point_light = PointLight::new(math::Vector3f::new(1.0, 1.0, 1.0), 10f32);
        point_light.transform.translation.z = 1f32;
        let lights = vec![point_light];

        GameState {
            objs,
            quad_mesh: crate::geo::screen_quad::gen_screen_quad(),
            lights,
        }
    }
}

pub struct GameApp {
    game_state: GameState,
    timer: Timer,
    input: InputState,
    window: Window,
    gl_context: GLContext,
    event_pump: EventPump,
    resources: Resources,
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
        let window = video_subsystem.window("gl_fun", window_width, window_height)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();
    
        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
        let mut event_pump = sdl_context.event_pump().unwrap();
    
        let mut resources = Resources::new();
        crate::load_assets::load_assets(&mut resources);

        GameApp {
            game_state: GameState::new(),
            timer,
            input,
            window,
            gl_context,
            event_pump,
            resources,
        }
    }
    pub fn game_loop(&mut self) {
        self.timer.start();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {

                        self.input.update(&event);

                    }
                }
            }
            self.timer.update();
            self.update();
            self.render();
            crate::gl_util::gl_dump_errors();
            self.window.gl_swap_window();
        }
    }
    pub fn update(&mut self) {
        self.game_state.update(self.timer.get_elapsed_ms();
    }
    pub fn render(&mut self) {
        for n in 0..self.game_state.render_objects.len() {

        }
    }
}