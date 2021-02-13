/*
Partially taken from
https://github.com/Nercury/rust-and-opengl-lessons
*/
use gl;
use std;
use std::ffi::{CString, CStr};

#[derive(Debug)]
pub enum Error {
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}


pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<Shader, Error> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, Error> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, Error> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    source: &CStr,
    kind: gl::types::GLenum
) -> Result<gl::types::GLuint, Error> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err( Error::CompileError{ name: "Compile error".to_string(), message: error.to_string_lossy().into_owned() });
    }

    Ok(id)
}

pub struct Program {
    pub id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shader1: &Shader, shader2: &Shader) -> Result<Program, Error> {
        let program_id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(program_id, shader1.id());
            gl::AttachShader(program_id, shader2.id());
            gl::LinkProgram(program_id);
            gl::DetachShader(program_id, shader1.id());
            gl::DetachShader(program_id, shader2.id());
        }
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(Error::LinkError {name: "Link error".to_string(), message: error.to_string_lossy().into_owned() });
        }
        return Ok( Program { id: program_id } );
    }
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn setMat4fv(&mut self, name: &str, data: *const f32) -> Option<()> {
        unsafe {
            let loc_u = gl::GetUniformLocation(self.id, std::ffi::CString::new(name).unwrap().as_ptr() as *const gl::types::GLchar);
            if loc_u == -1 {
                return Option::None;
            } else {
                self.set_used();
                gl::UniformMatrix4fv(loc_u, 1 as gl::types::GLsizei, gl::FALSE, data as *const gl::types::GLfloat);
            }
        }
        Some(())
    }
    pub fn set1i(&mut self, name: &str, data: i32) -> Option<()> {
        unsafe {
            let loc_u = gl::GetUniformLocation(self.id, std::ffi::CString::new(name).unwrap().as_ptr() as *const gl::types::GLchar);
            if loc_u == -1 {
                return Option::None;
            } else {
                self.set_used();
                gl::Uniform1i(loc_u, data);
            }
        }
        Some(())
    }
    pub fn set1f(&mut self, name: &str, data: f32) -> Option<()> {
        unsafe {
            let loc_u = gl::GetUniformLocation(self.id, std::ffi::CString::new(name).unwrap().as_ptr() as *const gl::types::GLchar);
            if loc_u == -1 {
                return Option::None;
            } else {
                self.set_used();
                gl::Uniform1f(loc_u, data);
            }
        }
        Some(())
    }
    pub fn set3f(&mut self, name: &str, x: f32, y: f32, z: f32) -> Option<()> {
        unsafe {
            let loc_u = gl::GetUniformLocation(self.id, std::ffi::CString::new(name).unwrap().as_ptr() as *const gl::types::GLchar);
            if loc_u == -1 {
                return Option::None;
            } else {
                self.set_used();
                gl::Uniform3f(loc_u, x, y, z);
            }
        }
        Some(())
    }
    pub fn set4f(&mut self, name: &str, x: f32, y: f32, z: f32, w: f32) -> Option<()> {
        unsafe {
            let loc_u = gl::GetUniformLocation(self.id, std::ffi::CString::new(name).unwrap().as_ptr() as *const gl::types::GLchar);
            if loc_u == -1 {
                return Option::None;
            } else {
                self.set_used();
                gl::Uniform4f(loc_u, x, y, z, w);
            }
        }
        Some(())
    }
    pub fn set3fv(&mut self, name: &str, count: i32, data: &[f32]) -> Option<()> {
        unsafe {
            let loc_u = gl::GetUniformLocation(self.id, std::ffi::CString::new(name).unwrap().as_ptr() as *const gl::types::GLchar);
            if loc_u == -1 {
                return Option::None;
            } else {
                self.set_used();
                gl::Uniform4fv(loc_u, count, data.as_ptr() as *const gl::types::GLfloat);
            }
        }
        Some(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}



fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}