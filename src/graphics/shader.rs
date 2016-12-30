extern crate glm;
extern crate gl;

use self::gl::types::*;
use std::fs::File;
use std::io::Read;
use std::io;
use std::fmt;
use std::error;
use std::mem;
use std::ffi::CString;
use std::ptr;
use std::str;

#[derive(Debug)]
pub enum ShaderError {
    IoError(io::Error),
    CompilationError(String),
    LinkError(String),
    InfoLogError(str::Utf8Error)
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderError::IoError(ref error) => write!(f, "IO Error: {}", error),
            ShaderError::CompilationError(ref error) => write!(f, "Shader compilation error: {}", error),
            ShaderError::LinkError(ref error) => write!(f, "Linking error: {}", error),
            ShaderError::InfoLogError(ref error) => write!(f, "InfoLog Error: {}", error),
        }
    }
}

impl error::Error for ShaderError {
    fn description(&self) -> &str {
        match *self {
            ShaderError::IoError(ref err) => err.description(),
            ShaderError::CompilationError(..) => "Error during shader compilation",
            ShaderError::LinkError(..) => "Error during linking",
            ShaderError::InfoLogError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ShaderError::IoError(ref error) => Some(error),
            ShaderError::InfoLogError(ref error) => Some(error),
            _ => None
        }
    }
}

impl From<io::Error> for ShaderError {
    fn from(err: io::Error) -> ShaderError {
        ShaderError::IoError(err)
    }
}

impl From<str::Utf8Error> for ShaderError {
    fn from(err: str::Utf8Error) -> ShaderError {
        ShaderError::InfoLogError(err)
    }
}

pub struct Shader
{
    pub vertex_path: String,
    pub fragment_path: String,
    pub program: GLuint
}

impl Shader
{
    pub fn new(vert_path: String, frag_path: String) -> Result<Shader, ShaderError> {
        let vertex_shader = try!(Shader::compile_shader(vert_path.as_str(), gl::VERTEX_SHADER));
        let fragment_shader = try!(Shader::compile_shader(frag_path.as_str(), gl::FRAGMENT_SHADER));
        let program = try!(Shader::link_program(vertex_shader, fragment_shader));

        Ok(Shader {
            vertex_path: vert_path,
            fragment_path: frag_path,
            program: program
        })
    }

    fn compile_shader(path: &str, shader_type: GLenum) -> Result<GLuint, ShaderError> {
        let mut shader_data = String::new();
        let mut file = try!(File::open(path));

        try!(file.read_to_string(&mut shader_data));

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

                return match str::from_utf8(&buf) {
                    Ok(msg) => Err(ShaderError::CompilationError(msg.to_string())),
                    Err(err) => Err(ShaderError::InfoLogError(err))
                }
            }
        }
        Ok(shader)
    }

    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<GLuint, ShaderError> {
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

                return match str::from_utf8(&buf) {
                    Ok(msg) => Err(ShaderError::LinkError(msg.to_string())),
                    Err(err) => Err(ShaderError::InfoLogError(err))
                }
            }

            Ok(program)
        }
    }

    pub fn use_program(&mut self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}
