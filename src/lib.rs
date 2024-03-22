// Window Creation + Control
use glfw::{Action, Key};
// Linear Algebra Crate
use ultraviolet;
use ultraviolet::projection::rh_yup::perspective_gl;

// Opening a window with an OpenGL conteext
pub mod window;
// Linking shaders to crete a GL Program
pub mod program;
pub use program::GLProgram;
// Compiling shaders into OpenGL programs
pub mod shader;
pub use shader::Shader;
// Loading shader, object, texture files
pub mod load;
// Creating and managing Vertex Array Objects
pub mod vao;
// Types and Setting Uniform Values
pub mod uniform;
// Re-export for more ergnomic use
pub mod types {
    pub use crate::uniform::*;
}

// This is used by GLPrograms to update their magic variables
pub struct FrameState {
    pub time: f32,                      // Total time elapsed
    pub resolution: Option<(f32, f32)>, // Width, Height
    pub perspective_matrix: ultraviolet::mat::Mat4,
}

pub fn frame_state(glfw: &glfw::Glfw) -> FrameState {
    FrameState {
        // time: if let Ok(elapsed) = time.elapsed() { elapsed.as_secs_f32() } else { 0.0 },
        time: glfw.get_time() as f32,
        resolution: None, // Only contains Some() when the screen changes size to avoid sending it
        // every frame
        perspective_matrix: perspective_gl(3.1415 / 6.0, 1.0, 1.0, -1.0),
    }
}

// Used in the render loop to set the FrameState
pub fn process_events(
    glfw: &glfw::Glfw,
    window: &mut glfw::PWindow,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) -> Result<FrameState, GLError> {
    let mut frame_state = frame_state(glfw);
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // Update Viewport, and Resolution Shader Uniform
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                frame_state.resolution = Some((width as f32, height as f32));
                let aspect_ratio = width as f32 / height as f32;
                frame_state.perspective_matrix = perspective_gl(1.0, aspect_ratio, 0.1, 100.0);
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
    Ok(frame_state)
}

// Our Errors will all roll up into this error type for easy handling
#[derive(Debug)]
pub enum GLError {
    Program(program::ProgramError),
    Shader(shader::ShaderError),
    Window(window::WindowError),
    Uniform(uniform::UniformError),
    VAO(vao::VAOError),
    Load(load::LoadError),
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
            GLError::Load(error) => {
                write!(f, "GL Program File Loading Error:\n{}", error.to_string())
            }
        }
    }
}
