use crate::framebuffer::{Framebuffer, GBuffer};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::render_object::{RenderObject};
use crate::resources::Resources;
use crate::geo::screen_quad::gen_screen_quad;
use crate::game::*;
use crate::transform::Transform;
use crate::material::*;
use crate::math::*;

pub struct Renderer {
    gbuffer: GBuffer,
    quad_screen_mesh: Mesh,
    window_width: u32,
    window_height: u32,
}

impl Renderer {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let mut quad_screen_mesh = gen_screen_quad();
        quad_screen_mesh.load();
        let gbuffer = GBuffer::new((window_width, window_height));

        Renderer {
            gbuffer,
            quad_screen_mesh,
            window_width,
            window_height,
        }
    }
    pub fn render(&self, res: &Resources, scene: &Scene) {
        unsafe {
            // g-buffer pass
            gl::Viewport(0, 0, self.window_width as i32, self.window_height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gbuffer.clear();
            self.gbuffer.set_as_target();
            let camera = &scene.camera;
            
            scene.player.draw(res, camera);


            for terrain_chunk in scene.terrain_chunks.iter() {
                terrain_chunk.draw(res, camera);
            }

            for target in scene.targets.iter() {
                target.draw(res, camera);
            }
            
            for obj in scene.objects.iter() {
                obj.draw(res, camera);
            }

            // lighting pass
            let mut light_pass_framebuffer = Framebuffer::create_with_attachments(vec![
                (gl::COLOR_ATTACHMENT0, (gl::RGBA32F, gl::RGBA, gl::FLOAT)), // position (vec3)/// hdr color
            ], (self.window_width, self.window_height));
            light_pass_framebuffer.bind();
            gl::ClearColor(0f32, 0f32, 0f32, 1f32);
            gl::Disable(gl::DEPTH_TEST);
            let mut lightpass_program = res.get_program_by_name("lightpass_program").unwrap();
            lightpass_program.set_used();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.get_position().textureID);
            lightpass_program.set1i("tposition", 0).unwrap();
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.get_normal().textureID);
            lightpass_program.set1i("tnormal", 1).unwrap();
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.get_albedo().textureID);
            lightpass_program.set1i("talbedo", 2).unwrap();
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.get_metallic().textureID);
            lightpass_program.set1i("tmetallic", 3).unwrap();
            gl::ActiveTexture(gl::TEXTURE4);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.get_roughness().textureID);
            lightpass_program.set1i("troughness", 4).unwrap();
            gl::ActiveTexture(gl::TEXTURE5);
            gl::BindTexture(gl::TEXTURE_2D, self.gbuffer.get_ao().textureID);
            lightpass_program.set1i("tao", 5).unwrap();
            
              //camera pos
            lightpass_program.set3f("camPos", camera.transform.translation.x, camera.transform.translation.y, camera.transform.translation.z);


            lightpass_program.set3f("lightPositions[0]", scene.lights[0].get_position().x, scene.lights[0].get_position().y, scene.lights[0].get_position().z);
            lightpass_program.set3f("lightColors[0]", scene.lights[0].get_color().x, scene.lights[0].get_color().y, scene.lights[0].get_color().z);
            lightpass_program.set3f("lightPositions[1]", scene.lights[1].get_position().x, scene.lights[1].get_position().y, scene.lights[1].get_position().z);
            lightpass_program.set3f("lightColors[1]", scene.lights[1].get_color().x, scene.lights[1].get_color().y, scene.lights[1].get_color().z);
            lightpass_program.set3f("lightPositions[2]", scene.lights[2].get_position().x, scene.lights[2].get_position().y, scene.lights[2].get_position().z);
            lightpass_program.set3f("lightColors[2]", scene.lights[2].get_color().x, scene.lights[2].get_color().y, scene.lights[2].get_color().z);
            lightpass_program.set3f("lightPositions[3]", scene.lights[3].get_position().x, scene.lights[3].get_position().y, scene.lights[3].get_position().z);
            lightpass_program.set3f("lightColors[3]", scene.lights[3].get_color().x, scene.lights[3].get_color().y, scene.lights[3].get_color().z);
            
            self.quad_screen_mesh.draw();

            // toss the lights in there
            {

                for light in scene.lights.iter() {

                    let program = res.get_program_by_name("flat_program").unwrap();
                    program.set_used();
                    let color = light.get_color();
                    program.setMat4fv("proj", camera.proj_mat().as_ptr()).unwrap();
                    program.setMat4fv("view", camera.view_mat().as_ptr()).unwrap();
                    program.setMat4fv("model", light.transform.model_mat().as_ptr()).unwrap();
                    program.set3f("color", color.x, color.y, color.z).unwrap();
                    let sphere_mesh = res.get_mesh_by_name("sphere").unwrap();
                    sphere_mesh.draw();
                }
            }
            // post-processing pass
            // tone-mapping
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            let tone_map_program = res.get_program_by_name("tone_map_program").unwrap();
            tone_map_program.set_used();
            let hdr_tex = light_pass_framebuffer.get_attachment(gl::COLOR_ATTACHMENT0).unwrap().textureID;
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, hdr_tex);
            tone_map_program.set1i("my_texture", 0);
            tone_map_program.set1f("exposure_bias", 8f32);
            tone_map_program.set3f("W", 0.9, 0.9, 0.9);
            self.quad_screen_mesh.draw();
            // hud pass
            let hud_program = res.get_program_by_name("hud_program").unwrap();
            hud_program.set_used();
            hud_program.set2f("iResolution", self.window_width as f32, self.window_height as f32);
            self.quad_screen_mesh.draw();
        }
    }
}