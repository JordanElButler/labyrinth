
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


pub fn main() {
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

    use crate::resources::Resources;
    let mut resources = Resources::new();
    crate::load_assets::load_assets(&mut resources);

    use std::path::{Path, PathBuf};
    let resource_path = loader::from_relative_exe_path(Path::new("assets")).unwrap();


    let mut camera = camera::PerspectiveCamera::new(80f32, (window_width as f32) / (window_height as f32), 0.01f32, 1000f32);


    use math::Vector2f;
    use crate::transform::{Transform};
    use render_object::RenderObject;

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

    let mut point_light = light::PointLight::new(math::Vector3f::new(1.0, 1.0, 1.0), 10f32);
    point_light.transform.translation.z = 1f32;

    let quad_verts = vec![
        -1f32,  1f32, 0f32, 1f32,
         1f32,  1f32, 1f32, 1f32,
        -1f32, -1f32, 0f32, 0f32,
         1f32, -1f32, 1f32, 0f32,
    ];
    let quad_ind = vec![
        0, 1, 2,
        2, 3, 1,
    ];

    let mut quad_vertex = vertex::Vertex::new(quad_verts, quad_ind, vertex::VertexLayout::new(vec![vertex::AttributeType::Position2D, vertex::AttributeType::ST]));
    quad_vertex.buffer_data();
    quad_vertex.set_attrib_pointers();


    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    // create some framebuffers
    
    let mut framebuffer: gl::types::GLuint = 0;

    let mut scene_texture: gl::types::GLuint = 0;
    let mut color_texture: gl::types::GLuint = 0;
    let mut normal_texture: gl::types::GLuint = 0;
    let mut depth_texture: gl::types::GLuint = 0;
    unsafe {
        gl::CreateFramebuffers(1, &mut framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

        gl::GenTextures(1, &mut scene_texture);
        gl::BindTexture(gl::TEXTURE_2D, scene_texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, window_width as i32, window_height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, scene_texture, 0);
        println!("{:?}", gl::CheckFramebufferStatus(gl::FRAMEBUFFER));
    /*
        gl::GenTextures(1, &mut color_texture);
        gl::BindTexture(gl::TEXTURE_2D, color_texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, window_width as i32, window_height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D, color_texture, 0);
    
        gl::GenTextures(1, &mut normal_texture);
        gl::BindTexture(gl::TEXTURE_2D, normal_texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, window_width as i32, window_height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT2, gl::TEXTURE_2D, normal_texture, 0);
    */
    
        gl::GenTextures(1, &mut depth_texture);
        gl::BindTexture(gl::TEXTURE_2D, depth_texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, window_width as i32, window_height as i32, 0, gl::DEPTH_COMPONENT, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_texture, 0);
    
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

    }

    let mut frames: i32 = 0;
    'running: loop {
        frames += 1;
        let mut p = Point{x:0.0, y: 0.0};
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if key == Keycode::A {
                        camera.transform.translation.x -= 0.1;
                    } else if key == Keycode::S {
                        camera.transform.translation.z -= 0.1;

                    } else if key == Keycode::D {
                        camera.transform.translation.x += 0.1;

                    } else if key == Keycode::W {
                        camera.transform.translation.z += 0.1;

                    } else if key == Keycode::Q {
                        camera.transform.rotation.y += 0.1;
                    } else if key == Keycode::E {
                        camera.transform.rotation.y -= 0.1;
                    } else if key == Keycode::R {
                        camera.transform.translation.y += 0.1;
                    } else if key == Keycode::F {
                        camera.transform.translation.y -= 0.1;

                    }
                }
                Event::MouseMotion {x, y, ..} => {
                    p.x = x as f64;
                    p.y = y as f64;
                },
                _ => {}
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        unsafe {
            point_light.transform.translation.y = 10f32 * (frames as f32/ 100f32).sin();

            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            gl::ClearColor(0f32, 1f32, 0f32, 1f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            first_obj.transform.rotation.y += 0.01;
            first_obj.draw(&mut resources, &camera, &point_light);
            second_obj.transform.rotation.y += 0.01;
            second_obj.draw(&mut resources, &camera, &point_light);
            third_obj.transform.rotation.y += 0.01;
            third_obj.draw(&mut resources, &camera, &point_light);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(1f32, 0f32, 0f32, 1f32);
            gl::Disable(gl::DEPTH_TEST);
            let quad_prog = resources.get_program("quad_program").unwrap();
            quad_prog.set_used();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, scene_texture);
            quad_prog.set1i("tex", 0);

            quad_vertex.bind();
            quad_vertex.draw_call();
        }

        gl_util::gl_dump_errors();

        window.gl_swap_window();


        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}