// Import our Error Type
pub mod error;
pub use error::WindowError;

use glfw::Context;

// Default window size
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// Default Window title
const WINDOW_TITLE: &str = "OpenGL";

// Default OpenGL API Version
const GL_MAJOR_VERSION: u32 = 4;
const GL_MINOR_VERSION: u32 = 6;

const MODE: glfw::WindowMode = glfw::WindowMode::Windowed;

// Type-Alias for readability
type GLFW = glfw::Glfw;
type GLWindow = glfw::PWindow;
type WindowEvents = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

// Creates and opens a new window, with specified dimensions, API version, and title
// Returns a handle to the window, and to the window's event loop
pub fn create<T: AsRef<str>>(
    title: T,
    width: u32,
    height: u32,
    gl_major_version: u32,
    gl_minor_version: u32,
) -> Result<(GLFW, GLWindow, WindowEvents), WindowError> {
    glfw::init(glfw::fail_on_errors)
        .and_then(|mut glfw| {
            // Set the version of OpenGL we're using
            glfw.window_hint(glfw::WindowHint::ContextVersion(
                gl_major_version,
                gl_minor_version,
            ));
            glfw.window_hint(glfw::WindowHint::OpenGlProfile(
                // Load the only the core, i.e. no extended, protocols
                glfw::OpenGlProfileHint::Core,
            ));
            // Don't allow use of deprecated features
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
            Ok(glfw)
        })
        .map_err(|_| WindowError::GLFWInitFailed)
        .and_then(|mut glfw| {
            glfw.create_window(width, height, title.as_ref(), MODE)
                .ok_or(WindowError::WindowCreateFailed)
                .and_then(|(window, events)| Ok((glfw, window, events)))
        })
        .and_then(|(glfw, mut window, events)| {
            // This function makes the OpenGL or OpenGL ES context of the specified window current on the calling thread
            window.make_current();
            // Notify us when a keyboard button is pressed
            window.set_key_polling(true);
            // Notify us when the window size (and therefore the frame buffer) changes
            window.set_framebuffer_size_polling(true);
            Ok((glfw, window, events))
        })
}

// Convenience function to open a standard sized window
pub fn create_default() -> Result<(GLFW, GLWindow, WindowEvents), WindowError> {
    create(
        WINDOW_TITLE,
        WIDTH,
        HEIGHT,
        GL_MAJOR_VERSION,
        GL_MINOR_VERSION,
    )
}
