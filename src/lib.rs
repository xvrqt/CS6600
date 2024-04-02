// Window Creation + Control
use glfw::{Action, Key};

// Opening a window with an OpenGL conteext
pub mod error;
pub use error::{GLError, GLUtilityError};
pub mod window;

// Linking shaders to crete a GL Program
pub mod program;
pub use program::builder;
pub use program::GLProgram;

pub use program::camera::CameraMove;
// Compiling shaders into OpenGL programs
pub mod shader;
pub use shader::Shader;
// Loading shader, object, texture files
pub mod load;
// Creating and managing Vertex Array Objects
pub mod vao;
// Types and Setting Uniform Values
pub mod uniform;
// Our Wavefront Obj parses into this
pub mod obj;

// Re-export for more ergnomic use
pub mod types {
    pub use crate::uniform::*;
}

// This is used by GLPrograms to update their magic variables
pub struct FrameState {
    pub time: f32,                      // Total time elapsed
    pub resolution: Option<(f32, f32)>, // Width, Height
    pub toggle_projection: bool,
    pub camera_change: std::vec::Vec<CameraMove>,
}

pub fn frame_state(glfw: &glfw::Glfw) -> FrameState {
    FrameState {
        // time: if let Ok(elapsed) = time.elapsed() { elapsed.as_secs_f32() } else { 0.0 },
        time: glfw.get_time() as f32,
        resolution: None, // Only contains Some() when the screen changes size to avoid sending it
        toggle_projection: false,
        camera_change: Vec::new(),
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
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(Key::P, _, Action::Press, _) => {
                frame_state.toggle_projection = true;
            }
            glfw::WindowEvent::Key(Key::W, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::Forwards);
            }
            glfw::WindowEvent::Key(Key::S, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::Backwards);
            }
            glfw::WindowEvent::Key(Key::Q, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::Left);
            }
            glfw::WindowEvent::Key(Key::E, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::Right);
            }
            glfw::WindowEvent::Key(Key::A | Key::H, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::LookLeft);
            }
            glfw::WindowEvent::Key(Key::D | Key::L, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::LookRight);
            }
            glfw::WindowEvent::Key(Key::J, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::LookDown);
            }
            glfw::WindowEvent::Key(Key::K, _, Action::Press | Action::Repeat, _) => {
                frame_state.camera_change.push(CameraMove::LookUp);
            }
            _ => {}
        }
    }
    Ok(frame_state)
}
