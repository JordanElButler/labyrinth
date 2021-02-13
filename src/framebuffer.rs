/*
    Wrapper for opengl framebuffer,
    can be used as render target
*/


use std::collections::HashMap;
use crate::texture::{Texture};

pub enum Error {

}

pub struct Framebuffer {
    id: gl::types::GLuint,
    tex_attachments: HashMap<gl::types::GLenum, Texture>,
    bound: bool,
}

impl Framebuffer {
    pub fn new() -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
        }

        Framebuffer {
            id: id,
            tex_attachments: HashMap::new(),
            bound: false,
        }
    }

    pub fn bind(&mut self) {
        self.bound = true;
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }
    pub fn unbind(&mut self) {
        self.bound = false;
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
    pub fn get_attachment(&self, attachment: gl::types::GLenum) -> Option<&Texture> {
        self.tex_attachments.get(&attachment)
    }
    pub fn create_with_attachments(attachments: Vec<(gl::types::GLenum, (gl::types::GLenum, gl::types::GLenum, gl::types::GLenum))>, dimensions: (u32, u32)) -> Self {
        let mut fb = Framebuffer::new();
        fb.bind();

        for attachment in attachments.iter() {
            let attachment_type = attachment.0;
            let format = attachment.1;
            let mut tex = Texture::create_with_attachment(format.0, format.1, format.2, dimensions, attachment_type);
            tex.bind();
            tex.load_memory();

            tex.set_parameters(vec![(gl::TEXTURE_MIN_FILTER, gl::LINEAR), (gl::TEXTURE_MAG_FILTER, gl::LINEAR), (gl::TEXTURE_WRAP_S, gl::REPEAT), (gl::TEXTURE_WRAP_T, gl::REPEAT)]);
            tex.unbind();

            unsafe {
                gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment_type, gl::TEXTURE_2D, tex.textureID, 0);
            }
            fb.tex_attachments.insert(attachment_type, tex);


        }

        unsafe {
            println!("{:?} on {}", gl::CheckFramebufferStatus(gl::FRAMEBUFFER), std::line!());
        }
        fb.unbind();

        fb
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}

pub struct GBuffer {
    fb: Framebuffer
}

impl GBuffer {
    pub fn new(dimensions: (u32, u32)) -> Self {
        GBuffer {
            fb: Framebuffer::create_with_attachments(vec![
                (gl::COLOR_ATTACHMENT0, (gl::RGBA32F, gl::RGBA, gl::FLOAT)), // position (vec3)
                (gl::COLOR_ATTACHMENT1, (gl::RGBA32F, gl::RGBA, gl::FLOAT)), // normal (vec3) non-normalized
                (gl::COLOR_ATTACHMENT2, (gl::RGBA32F, gl::RGBA, gl::FLOAT)), // albedo (vec3)
                (gl::COLOR_ATTACHMENT3, (gl::R32F, gl::RED, gl::FLOAT)), // metallic (float)
                (gl::COLOR_ATTACHMENT4, (gl::R32F, gl::RED, gl::FLOAT)), // roughness (float)
                (gl::COLOR_ATTACHMENT5, (gl::R32F, gl::RED, gl::FLOAT)), // ao (float)
                (gl::DEPTH_ATTACHMENT, (gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT, gl::UNSIGNED_BYTE)) // depth
            ], dimensions)
        }
    }
    pub fn set_as_target(&mut self) {
        self.fb.bind();
        unsafe {

            gl::Enable(gl::DEPTH_TEST);
            gl::DrawBuffers(6, &[gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2, gl::COLOR_ATTACHMENT3, gl::COLOR_ATTACHMENT4, gl::COLOR_ATTACHMENT5] as *const gl::types::GLenum);
        }
    }
    pub fn clear(&mut self) {
        self.fb.bind();
        unsafe {
            gl::ClearColor(0f32, 0f32, 0f32, 1f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn get_position(&self) -> &Texture {
        self.fb.get_attachment(gl::COLOR_ATTACHMENT0).unwrap()
    }
    pub fn get_normal(&self) -> &Texture {
        self.fb.get_attachment(gl::COLOR_ATTACHMENT1).unwrap()
    }
    pub fn get_albedo(&self) -> &Texture {
        self.fb.get_attachment(gl::COLOR_ATTACHMENT2).unwrap()
    }
    pub fn get_metallic(&self) -> &Texture {
        self.fb.get_attachment(gl::COLOR_ATTACHMENT3).unwrap()
    }
    pub fn get_roughness(&self) -> &Texture {
        self.fb.get_attachment(gl::COLOR_ATTACHMENT4).unwrap()
    }
    pub fn get_ao(&self) -> &Texture {
        self.fb.get_attachment(gl::COLOR_ATTACHMENT5).unwrap()
    }
    pub fn get_depth(&self) -> &Texture {
        self.fb.get_attachment(gl::DEPTH_ATTACHMENT).unwrap()

    }
}
