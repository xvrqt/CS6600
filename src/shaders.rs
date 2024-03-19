use gl::types::*;

// We're calling into the user's OpenGL Library
use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::str;

#[derive(Debug)]
pub enum Shader {
    VertexShader { id: GLuint, source: String },
    FragmentShader { id: GLuint, source: String },
}

#[derive(Debug)]
pub enum ShaderError {
    SourceParse,
    SourceCompilation(String),
    UnknownType,
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderError::SourceParse => {
                write!(f, "Failed to convert the shader's source into a c-string.")
            }
            ShaderError::SourceCompilation(error_log) => write!(
                f,
                "Failed to compile the shader from source.\n{}",
                error_log
            ),
            ShaderError::UnknownType => write!(
                f,
                "Could not determine shader type. How did you even get this error."
            ),
        }
    }
}

impl Shader {
    // Creates and compiles a new GLSL shader of the specified type
    fn new(shader_type: GLenum, source: &str) -> Result<Self, ShaderError> {
        // Hoisted to make the construction more readable at the end
        let shader;
        // Allocation a new string for 'source' since we're saving it anyways
        // TODO: Could the be a COW ?
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

        match shader_type {
            gl::VERTEX_SHADER => Ok(Shader::VertexShader { id: shader, source }),
            gl::FRAGMENT_SHADER => Ok(Shader::FragmentShader { id: shader, source }),
            _ => Err(ShaderError::UnknownType),
        }
    }

    // Returns the ID of a shader (used by OpenGL to specify it)
    // Convenience to save us destructuring it later
    fn id(&self) -> GLuint {
        match self {
            Shader::VertexShader { id, .. } => *id,
            Shader::FragmentShader { id, .. } => *id,
        }
    }

    // Tell OpenGL we don't need the shader around anymore
    fn delete(self) -> () {
        unsafe {
            gl::DeleteShader(self.id());
        }
    }
}

#[derive(Debug)]
pub struct GLPROGRAM {
    vertex_shader: Shader,
    fragment_shader: Shader,
    program: u32,
}

#[derive(Debug)]
pub enum GLProgramError {
    ShaderCompilation(ShaderError),
    Linking(String),
}

impl fmt::Display for GLProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GLProgramError::ShaderCompilation(shader_error) => {
                write!(f, "Error in processing a shader.\n{}", shader_error)
            }
            GLProgramError::Linking(error_log) => {
                write!(f, "Could not link shaders to the program.\n{}", error_log)
            }
        }
    }
}

impl GLPROGRAM {
    pub fn new(vs: &str, fs: &str) -> Result<Self, GLProgramError> {
        let vertex_shader =
            Shader::new(gl::VERTEX_SHADER, vs).map_err(|e| GLProgramError::ShaderCompilation(e))?;
        let fragment_shader = Shader::new(gl::FRAGMENT_SHADER, fs)
            .map_err(|e| GLProgramError::ShaderCompilation(e))?;

        // Link the shaders to the program
        let program; // Hoisted for readability
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader.id());
            gl::AttachShader(program, fragment_shader.id());
            gl::LinkProgram(program);

            //
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                // Determine the log's length
                let mut length = 0 as GLint;
                gl::GetShaderiv(program, gl::INFO_LOG_LENGTH, &mut length);
                let log_length: usize = length.try_into().map_err(|_| {
                    GLProgramError::Linking(String::from("Couldn't determine length of error log."))
                })?;

                // Set up a buffer to receive the log
                let mut error_log = Vec::<u8>::with_capacity(log_length);
                error_log.set_len(log_length - 1); // Don't read the NULL terminator
                gl::GetProgramInfoLog(
                    program,
                    512,
                    ptr::null_mut(),
                    error_log.as_mut_ptr() as *mut GLchar,
                );

                // Return the error log and exit
                return Err(GLProgramError::Linking(
                    str::from_utf8(&error_log).unwrap().into(),
                ));
            }
        }

        Ok(GLPROGRAM {
            program,
            vertex_shader,
            fragment_shader,
        })
    }

    pub fn id(&self) -> GLuint {
        self.program
    }
}
