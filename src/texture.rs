
#[derive(Debug)]
pub struct Texture {
    pub internal_format: gl::types::GLenum,
    pub format: gl::types::GLenum,
    pub data_type: gl::types::GLenum,
    pub dimensions: (u32, u32),
    pub texture_type: TextureType,
    pub textureID: gl::types::GLuint,
    pub is_loaded: bool,
}

impl Texture {
    pub fn create_with_attachment(internal_format: gl::types::GLenum, format: gl::types::GLenum, data_type: gl::types::GLenum, dimensions: (u32, u32), attachment_type: gl::types::GLenum) -> Self {
        let mut textureID: gl::types::GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut textureID);
        }
        Texture {
            internal_format,
            format,
            data_type,
            dimensions,
            texture_type: TextureType::FramebufferAttachment(attachment_type),
            textureID: textureID,
            is_loaded: false,
        }
    }
    pub fn create_with_data(internal_format: gl::types::GLenum, format: gl::types::GLenum, data_type: gl::types::GLenum, dimensions: (u32, u32), bytes: Vec<u8>) -> Self {
        let mut textureID: gl::types::GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut textureID);
        }
        Texture {
            internal_format,
            format,
            data_type,
            dimensions,
            texture_type: TextureType::Data(bytes),
            textureID: textureID,
            is_loaded: false,
        }
    }

    // TODO: make color format flexible
    pub fn load_memory(&mut self) {
        match &self.texture_type {
            TextureType::FramebufferAttachment(_) => {
                self.bind();
                unsafe {
                    gl::TexImage2D(gl::TEXTURE_2D, 0, self.internal_format as i32, self.dimensions.0 as gl::types::GLint, self.dimensions.1 as gl::types::GLint, 0, self.format, self.data_type, std::ptr::null() as *const gl::types::GLvoid);
                }
                self.unbind();
            },
            TextureType::Data(bytes) => {
                self.bind();
                unsafe {
                    gl::TexImage2D(gl::TEXTURE_2D, 0, self.internal_format as i32, self.dimensions.0 as gl::types::GLint, self.dimensions.1 as gl::types::GLint, 0, self.format, self.data_type, bytes.as_ptr() as *const gl::types::GLvoid);
                }
                self.unbind();
            }
        }
    }
    pub fn set_parameters(&mut self, parameters: Vec<(gl::types::GLenum, gl::types::GLenum)>) {
        self.bind();
        unsafe {
            for i in 0..parameters.len() {
                let para = parameters.get(i).unwrap();
                gl::TexParameteri(gl::TEXTURE_2D, para.0, para.1 as gl::types::GLint);
            }
        }
        self.unbind();
    }
    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.textureID);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl std::fmt::Display for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.format, self.dimensions)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.textureID);
        }
    }
}

#[derive(Debug)]
pub enum TextureType {
    Data(Vec<u8>),
    FramebufferAttachment(gl::types::GLenum),
}