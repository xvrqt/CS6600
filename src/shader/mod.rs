// Import our ShaderError types
pub mod error;
pub use error::ShaderError;

// Import our built-in shader types
pub mod blinn_phong;
pub use blinn_phong::{BlinnPhongFragmentShader, BlinnPhongVertexShader};

use gl::types::*;

// We're calling into the user's OpenGL Library
use std::ffi::CString;
use std::ptr;
use std::str;

// Dummy types to help the compiler catch mistakes
#[derive(Debug)]
pub struct Vertex;
#[derive(Debug)]
pub struct Fragment;

// Nice type alias denoting the use of not built-in shaders
pub type CustomVertexShader<'a> = Shader<'a, Vertex>;
pub type CustomFragmentShader<'a> = Shader<'a, Fragment>;

// All shaders must have a defined type corresponding to the part of the graphics
// pipeline they operate on. Making them an Enum also provides easy matching,
// and prevents accidentally assigning a Geometry Shader to the Vertex Shader
#[derive(Debug)]
pub struct Shader<'a, Type> {
    pub(crate) id: gl::types::GLuint,
    // If we have a 'static str as our shader code (likely during rapid development)
    // Then we skip an allocation
    #[allow(dead_code)]
    pub(crate) source: std::borrow::Cow<'a, str>,
    _pd: std::marker::PhantomData<Type>,
}

// Different types of shaders. Vertex, Fragment are mandatory
impl<'a> Shader<'a, Vertex> {
    pub fn new(source: &'a str) -> Result<CustomVertexShader, ShaderError> {
        new_shader::<Vertex>(source)
    }

    pub fn blinn_phong() -> Result<BlinnPhongVertexShader<'a>, ShaderError> {
        let vs = new_shader::<Vertex>(blinn_phong::VERTEX_SHADER_SOURCE)?;
        Ok(BlinnPhongVertexShader(vs))
    }
}

impl<'a> Shader<'a, Fragment> {
    pub fn new(source: &'a str) -> Result<CustomFragmentShader, ShaderError> {
        new_shader::<Fragment>(source)
    }

    pub fn blinn_phong() -> Result<BlinnPhongFragmentShader<'a>, ShaderError> {
        let fs = new_shader::<Fragment>(blinn_phong::FRAGMENT_SHADER_SOURCE)?;
        Ok(BlinnPhongFragmentShader(fs))
    }
}

// When we're done with the shader, let OpenGL know it can clean it up
impl<'a, Type> Drop for Shader<'a, Type> {
    // Tell OpenGL we don't need the shader around anymore
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

trait ShaderTypes {
    fn gl_shader_type() -> GLuint;
}
impl ShaderTypes for Vertex {
    fn gl_shader_type() -> GLuint {
        gl::VERTEX_SHADER as GLuint
    }
}
impl ShaderTypes for Fragment {
    fn gl_shader_type() -> GLuint {
        gl::FRAGMENT_SHADER
    }
}

// Conveninece function called by all shader types
// Uses the 'Type' to correctly call GL with the correspond shader type enum
// This means that 'Type's must implement the trait "ShaderTypes"
fn new_shader<'a, Type>(source: &'a str) -> Result<Shader<Type>, ShaderError>
where
    Type: ShaderTypes,
{
    // Hoisted to make the construction more readable at the end
    let shader;
    let shader_type = Type::gl_shader_type();
    // Allocation a new string for 'source' since we're saving it anyways
    let source = String::from(source);
    let src_c_str = CString::new(source.as_bytes()).map_err(|_| ShaderError::SourceParse)?;
    unsafe {
        // Aske OpenGL for a new shader, and attempt to compile the source
        shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &src_c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            // Check if the shader compiled, and save the error log if not
            // Determine how long the log is
            let mut length = 0 as GLint;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
            let log_length: usize = length.try_into().map_err(|_| {
                ShaderError::SourceCompilation(String::from(
                    "Couldn't determine length of error log.",
                ))
            })?;
            // Set up a buffer to receive the log
            let mut error_log = Vec::<u8>::with_capacity(log_length);
            error_log.set_len(log_length - 1); // Don't read the NULL terminator

            // Actually get the log itself lol
            gl::GetShaderInfoLog(
                shader,
                512,
                ptr::null_mut(),
                error_log.as_mut_ptr() as *mut GLchar,
            );

            // Return the error log and exit
            return Err(ShaderError::SourceCompilation(
                str::from_utf8(&error_log).unwrap().into(),
            ));
        }
    }

    Ok(Shader {
        id: shader,
        source: source.into(),
        _pd: std::marker::PhantomData::<Type>,
    })
}
