// Import Crate Error Types
pub mod error;
use crate::error::GLUtilityError;
pub use error::ShaderError;
mod fragment_only;
type Result<T> = std::result::Result<T, error::ShaderError>;

// Import our built-in shader types
pub mod blinn_phong;

// OpenGL Types
use gl::types::*;

// We're calling into the user's OpenGL Library, so we need to work with raw strings and pointers
use std::ffi::CString;
use std::ptr;
use std::str;

// Dummy types to help the compiler catch mistakes
#[derive(Debug)]
pub struct VertexShader;
#[derive(Debug)]
pub struct FragmentShader;
#[derive(Debug)]
pub struct GeometryShader;
#[derive(Debug)]
pub struct TesselationShader;

// All shaders must have a defined type corresponding to the part of the graphics pipeline they
// operate on, and prevents accidentally assigning a Geometry Shader to the Vertex Shader
#[derive(Debug)]
pub struct Shader<'a, Type> {
    pub(crate) id: gl::types::GLuint,
    // If we have a 'static str as our shader code (e.g. when using a built-in shader), then we
    // skip an allocation. Not used, but pretty sure I'll use it eventually lol. #YAGNI
    #[allow(dead_code)]
    source: std::borrow::Cow<'a, str>,
    _pd: std::marker::PhantomData<Type>,
}

#[derive(Debug)]
pub(crate) struct ShaderPipeline<'a> {
    pub(crate) vertex_shader: Shader<'a, VertexShader>,
    pub(crate) fragment_shader: Shader<'a, FragmentShader>,
    pub(crate) geometry_shader: Option<Shader<'a, GeometryShader>>,
    pub(crate) tessellation_shader: Option<Shader<'a, TesselationShader>>,
}

impl<'a> ShaderPipeline<'a> {
    pub(crate) fn new(
        program_id: GLuint,
        vertex_shader: Shader<'a, VertexShader>,
        fragment_shader: Shader<'a, FragmentShader>,
        geometry_shader: Option<Shader<'a, GeometryShader>>,
        tessellation_shader: Option<Shader<'a, TesselationShader>>,
    ) -> Result<Self> {
        unsafe {
            gl::AttachShader(program_id, vertex_shader.id);
            gl::AttachShader(program_id, fragment_shader.id);

            if let Some(ref geometry_shader) = geometry_shader {
                gl::AttachShader(program_id, geometry_shader.id);
            }
            if let Some(ref tessellation_shader) = tessellation_shader {
                gl::AttachShader(program_id, tessellation_shader.id);
            }

            gl::LinkProgram(program_id);
        }
        // Check that all went well, and return a new ShaderPipline if so
        link_shaders_success(program_id).map(|_| ShaderPipeline {
            vertex_shader,
            fragment_shader,
            geometry_shader,
            tessellation_shader,
        })
    }
}

// Different types of shaders. Vertex, Fragment are mandatory
impl<'a> Shader<'a, VertexShader> {
    pub fn new(source: &'a str) -> Result<Shader<'a, VertexShader>> {
        Self::new_shader(source, gl::VERTEX_SHADER)
    }

    pub fn blinn_phong() -> Result<Shader<'a, VertexShader>> {
        Self::new_shader(blinn_phong::VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)
    }

    pub fn fragment_only() -> Result<Shader<'a, VertexShader>> {
        Self::new_shader(fragment_only::VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)
    }
}

impl<'a> Shader<'a, FragmentShader> {
    pub fn new(source: &'a str) -> Result<Shader<'a, FragmentShader>> {
        Self::new_shader(source, gl::FRAGMENT_SHADER)
    }

    pub fn blinn() -> Result<Shader<'a, FragmentShader>> {
        Self::new_shader(
            blinn_phong::BLINN_FRAGMENT_SHADER_SOURCE,
            gl::FRAGMENT_SHADER,
        )
    }

    pub fn phong() -> Result<Shader<'a, FragmentShader>> {
        Self::new_shader(
            blinn_phong::PHONG_FRAGMENT_SHADER_SOURCE,
            gl::FRAGMENT_SHADER,
        )
    }
}

impl<'a, Type> Shader<'a, Type> {
    // Create a new shader, of a specified 'Type'
    fn new_shader(source: &'a str, shader_type: GLuint) -> Result<Shader<Type>> {
        // Hoisted to make the construction more readable at the end
        let shader;
        let src_c_str = CString::new(source).map_err(|_| {
            ShaderError::FailedToParseSource(GLUtilityError::FailedToConvertToCString(
                source.to_string(),
            ))
        })?;

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
                    ShaderError::FailedToCompileShader(GLUtilityError::CouldNotCreateErrorLog)
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

                let log = str::from_utf8(&error_log).unwrap_or_else(|error| {
                    str::from_utf8(&error_log[..error.valid_up_to()])
                        .unwrap()
                        .into()
                });

                // Return the error log and exit
                return Err(ShaderError::FailedToCompileShader(
                    GLUtilityError::ErrorLog(log.to_string()),
                ));
            }
        }

        Ok(Shader {
            id: shader,
            source: source.into(),
            _pd: std::marker::PhantomData::<Type>,
        })
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

// Helper function that checks if linking the shaders to the program was a success
pub(crate) fn link_shaders_success(program_id: GLuint) -> Result<()> {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            // Determine the log's length
            let mut length = 0 as GLint;
            gl::GetShaderiv(program_id, gl::INFO_LOG_LENGTH, &mut length);
            let log_length: usize = length.try_into().map_err(|_| {
                ShaderError::FailedToLinkShaders(GLUtilityError::CouldNotCreateErrorLog)
            })?;

            // Set up a buffer to receive the log
            let mut error_log = Vec::<u8>::with_capacity(log_length);
            if log_length > 0 {
                error_log.set_len(log_length - 1);
            } // Don't read the NULL terminator

            gl::GetProgramInfoLog(
                program_id,
                512,
                std::ptr::null_mut(),
                error_log.as_mut_ptr() as *mut GLchar,
            );

            // Return the error log and exit
            Err(ShaderError::FailedToLinkShaders(GLUtilityError::ErrorLog(
                std::str::from_utf8(&error_log).unwrap().into(),
            )))
        } else {
            Ok(())
        }
    }
}

// Loads a shader from a path, nothing special
pub fn load_shader<P>(path: P) -> Result<String>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    std::fs::read_to_string(path).map_err(|io_error| {
        ShaderError::FailedToLoadSource(GLUtilityError::CouldNotOpenFile(
            path.to_string_lossy().to_string(),
            io_error,
        ))
    })
}
