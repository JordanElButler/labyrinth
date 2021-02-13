pub mod input;
pub mod timer;
pub mod shader;
pub mod math;
pub mod camera;
pub mod transform;
pub mod mesh;
pub mod loader;
pub mod texture;
pub mod render_object;
pub mod vertex;
pub mod gl_util;
pub mod geo;
pub mod resources;
pub mod load_assets;
pub mod light;
pub mod framebuffer;
pub mod material;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Texture;
use sdl2::video::GLProfile;

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

    let mut camera = camera::PerspectiveCamera::new(80f32, (window_width as f32) / (window_height as f32), 0.01f32, 100f32);

    use math::{Vector3f, Vector2f};
    use crate::transform::{Transform};
    use render_object::RenderObject;
    use crate::material::Material;

    let mut objs: Vec<RenderObject> = Vec::new();

    for x in 0..10 {
        for y in 0..10 {
            let mut transform = Transform::identity();
            transform.translation.x = (x - 5) as f32;
            transform.translation.y = (y - 5) as f32;
            transform.translation.z = 5 as f32;

            transform.scale.x = 0.5;
            transform.scale.y = 0.5;
            transform.scale.z = 0.5;

            objs.push( RenderObject::new(
                transform,
                "g_program",
                "sphere",
                Material::make_basic_material(
                    Vector3f::new(1f32, 0f32, 0f32),
                    x as f32 / 10.0,
                    y as f32 / 10.0,
                    0.2f32)));
        }
    }

    use light::PointLight;
    let mut lights: Vec<PointLight> = Vec::new();
    for x in 0..2 {
        for y in 0..2 {
            let mut transform = Transform::identity();
            transform.translation.x = (4 * (x - 1)) as f32;
            transform.translation.y = (4 * (y - 1))  as f32;
            lights.push( PointLight::new(
                transform,
                Vector3f::new(1.0, 1.0, 1.0),
                1.0
            ));
        }
    }

    let mut quad_screen_mesh = geo::screen_quad::gen_screen_quad();
    quad_screen_mesh.load();
    // create some framebuffers
    use crate::framebuffer::{GBuffer, Framebuffer};
    let mut my_gb = GBuffer::new((window_width, window_height));


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

            // g-buffer pass
            gl::Viewport(0, 0, window_width as i32, window_height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            my_gb.clear();
            my_gb.set_as_target();
            for obj in objs.iter_mut() {
                obj.draw(&mut resources, &camera);
            }
 /*          
            // show normals
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Disable(gl::DEPTH_TEST);
            let quad_program = resources.get_program("quad_program").unwrap();
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, my_gb.get_metallic().textureID);
            quad_program.set1i("my_texture", 0).unwrap();
            quad_screen_mesh.draw();
            gl_util::gl_dump_errors();
*/
            // lighting pass
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(0f32, 0f32, 0f32, 1f32);
            gl::Disable(gl::DEPTH_TEST);
            let mut lightpass_program = resources.get_program("lightpass_program").unwrap();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, my_gb.get_position().textureID);
            lightpass_program.set1i("tposition", 0).unwrap();
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, my_gb.get_normal().textureID);
            lightpass_program.set1i("tnormal", 1).unwrap();
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, my_gb.get_albedo().textureID);
            lightpass_program.set1i("talbedo", 2).unwrap();
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, my_gb.get_metallic().textureID);
            lightpass_program.set1i("tmetallic", 3).unwrap();
            gl::ActiveTexture(gl::TEXTURE4);
            gl::BindTexture(gl::TEXTURE_2D, my_gb.get_roughness().textureID);
            lightpass_program.set1i("troughness", 4).unwrap();
            gl::ActiveTexture(gl::TEXTURE5);
            gl::BindTexture(gl::TEXTURE_2D, my_gb.get_ao().textureID);
            lightpass_program.set1i("tao", 5).unwrap();



            
              //camera pos
            lightpass_program.set3f("camPos", camera.transform.translation.x, camera.transform.translation.y, camera.transform.translation.z);

            // lights
            println!("{}", frames);
            gl_util::gl_dump_errors();

            let lightPositions = [
                -2f32, -2f32, 0f32, 
                -2f32, 2f32, 0f32, 
                2f32, -2f32, 0f32,
                2f32, 2f32, 0f32];
            let lightColors = [
                1f32, 1f32, 1f32,
                1f32, 1f32, 1f32,
                1f32, 1f32, 1f32,
                1f32, 1f32, 1f32,
            ];

            lightpass_program.set3f("lightPositions[0]", lightPositions[ 0 * 3 + 0], lightPositions[ 0 * 3 + 1], lightPositions[ 0 * 3 + 2]);
            lightpass_program.set3f("lightColors[0]", lightColors[ 0 * 3 + 0], lightColors[ 0 * 3 + 1], lightColors[ 0 * 3 + 2]);
            lightpass_program.set3f("lightPositions[1]", lightPositions[ 1 * 3 + 0], lightPositions[ 1 * 3 + 1], lightPositions[ 1 * 3 + 2]);
            lightpass_program.set3f("lightColors[1]", lightColors[ 1 * 3 + 0], lightColors[ 1 * 3 + 1], lightColors[ 1 * 3 + 2]);
            lightpass_program.set3f("lightPositions[2]", lightPositions[ 2 * 3 + 0], lightPositions[ 2 * 3 + 1], lightPositions[ 2 * 3 + 2]);
            lightpass_program.set3f("lightColors[2]", lightColors[ 2 * 3 + 0], lightColors[ 2 * 3 + 1], lightColors[ 2 * 3 + 2]);
            lightpass_program.set3f("lightPositions[3]", lightPositions[ 3 * 3 + 0], lightPositions[ 3 * 3 + 1], lightPositions[ 3 * 3 + 2]);
            lightpass_program.set3f("lightColors[3]", lightColors[ 3 * 3 + 0], lightColors[ 3 * 3 + 1], lightColors[ 3 * 3 + 2]);


            quad_screen_mesh.draw();
            // post-processing pass

            // final pass
  //*/
        }


        window.gl_swap_window();

//        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

struct Point {
    x: f64,
    y: f64,
}
