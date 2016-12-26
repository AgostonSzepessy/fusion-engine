extern crate glm;
extern crate gl;

use self::gl::types::*;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ffi::CString;
use std::ptr;
use std::str;

pub struct Shader
{
    pub vertex_path: String,
    pub fragment_path: String,
    pub program: GLuint
}

impl Shader
{
    pub fn new(vert_path: String, frag_path: String) -> Shader {
        let vertex_shader = Shader::compile_shader(vert_path.as_str(), gl::VERTEX_SHADER);
        let fragment_shader = Shader::compile_shader(frag_path.as_str(), gl::FRAGMENT_SHADER);
        let program = Shader::link_program(vertex_shader, fragment_shader);

        Shader {
            vertex_path: vert_path,
            fragment_path: frag_path,
            program: program
        }
    }

    fn compile_shader(path: &str, shader_type: GLenum) -> GLuint {
        let mut shader_data = String::new();
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error while reading shader: {}", path);
                panic!("Error is {}", e);
            }
        };

        file.read_to_string(&mut shader_data).expect("Unable to read shader data from shader");

        let shader;

        unsafe {
            shader = gl::CreateShader(shader_type);
            let shader_src = CString::new(shader_data.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &shader_src.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
            }
        }
        shader
    }

    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint{
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // get link status
            let mut status = gl::FALSE as GLint;

            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
            }

            program
        }
    }

    pub fn use_program(&mut self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}
