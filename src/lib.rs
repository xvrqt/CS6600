// Compiling shaders into OpenGL programs
pub mod shader;
pub use shader::Shader;
// Opening a window with an OpenGL conteext
pub mod window;
// Linking shaders to crete a GL Program
pub mod program;
pub use program::GLProgram;

// Types and Setting Uniform Values
pub mod uniform;
pub mod types {
    pub use crate::uniform::*;
}

// Our Errors will all roll up into this error type for easy handling
#[derive(Debug)]
pub enum GLError {
    Program(program::ProgramError),
    Shader(shader::ShaderError),
    Window(window::WindowError),
    Uniform(uniform::UniformError),
}

impl std::error::Error for GLError {}

impl std::fmt::Display for GLError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GLError::Program(error) => {
                write!(f, "GL Program Error:\n{}", error.to_string())
            }
            GLError::Shader(error) => {
                write!(f, "GL Shader Error:\n{}", error.to_string())
            }
            GLError::Window(error) => {
                write!(f, "GL Window Error:\n{}", error.to_string())
            }
            GLError::Uniform(error) => {
                write!(f, "GL Uniform Assignment Error:\n{}", error.to_string())
            }
        }
    }
}
