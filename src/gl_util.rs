/*
dumpster function
*/
pub fn gl_dump_errors() {
    loop {
        let mut error_code: gl::types::GLenum = 0;
        unsafe {
            error_code = gl::GetError();
        }

        if error_code == gl::NO_ERROR {
            break;
        }
        match error_code {
            gl::INVALID_ENUM => println!("GL: Invalid enum"),
            gl::INVALID_VALUE => println!("GL: Invalid value"),
            gl::INVALID_OPERATION => println!("GL: Invalid operation"),
            gl::STACK_OVERFLOW => println!("GL: Stack overflow"),
            gl::STACK_UNDERFLOW => println!("GL: Stack underflow"),
            gl::OUT_OF_MEMORY => println!("GL: Out of memory"),
            gl::INVALID_FRAMEBUFFER_OPERATION => println!("GL: Invalid framebuffer operation"),
            _ => println!("GL: Unknown error!"),
        }
    }
}