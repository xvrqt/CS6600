// Compiling shaders into OpenGL programs
pub mod shader;
pub use shader::Shader;
// Opening a window with an OpenGL conteext
pub mod window;
// Linking shaders to crete a GL Program
pub mod program;
pub use program::GLProgram;

// Our Errors will all roll up into this error type for easy handling
#[derive(Debug)]
pub enum GLError {
    Program(program::ProgramError),
    Shader(shader::ShaderError),
    Window(window::WindowError),
}

impl std::error::Error for GLError {}

impl std::fmt::Display for GLError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GLError::Shader(error) => {
                write!(f, "GL Shader Error:\n{}", error.to_string())
            }
            GLError::Program(error) => {
                write!(f, "GL Program Error:\n{}", error.to_string())
            }
            GLError::Window(error) => {
                write!(f, "GL Window Error:\n{}", error.to_string())
            }
        }
    }
}
