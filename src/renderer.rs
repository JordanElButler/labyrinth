use crate::framebuffer::{GBuffer};
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::render_object::{RenderObject, Scene};
use crate::resources::Resources;
use crate::geo::screen_quad::gen_screen_quad;

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
    pub fn render(&self, res: &Resources, scene: &Scene, camera: &Camera) {
        unsafe {
            // g-buffer pass
            gl::Viewport(0, 0, self.window_width as i32, self.window_height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gbuffer.clear();
            self.gbuffer.set_as_target();
            for obj in scene.objects.iter() {
                obj.draw(res, &camera);
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
            let mut lightpass_program = res.get_program_by_name("lightpass_program").unwrap();
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

            self.quad_screen_mesh.draw();
            // post-processing pass
              // tone-mapping

            // final pass
        }
    }
}