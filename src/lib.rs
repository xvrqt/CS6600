// Compiling shaders into OpenGL programs
pub mod shader;
pub use shader::Shader;
// Opening a window with an OpenGL conteext
pub mod window;
// Linking shaders to crete a GL Program
pub mod program;
pub use program::GLProgram;

pub mod vao;

// Types and Setting Uniform Values
pub mod uniform;
pub mod types {
    pub use crate::uniform::*;
}

// This is used by GLPrograms to update their magic variables
pub struct FrameState {
    pub time: f32,                      // Total time elapsed
    pub resolution: Option<(f32, f32)>, // Width, Height
}

pub fn frame_state(glfw: &glfw::Glfw) -> FrameState {
    FrameState {
        // time: if let Ok(elapsed) = time.elapsed() { elapsed.as_secs_f32() } else { 0.0 },
        time: glfw.get_time() as f32,
        resolution: None, // Only contains Some() when the screen changes size to avoid sending it
                          // every frame
    }
}

// Our Errors will all roll up into this error type for easy handling
#[derive(Debug)]
pub enum GLError {
    Program(program::ProgramError),
    Shader(shader::ShaderError),
    Window(window::WindowError),
    Uniform(uniform::UniformError),
    VAO(vao::VAOError),
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
            GLError::VAO(error) => {
                write!(f, "GL Attribute Creation Error:\n{}", error.to_string())
            }
        }
    }
}
