// Import our Error Type
pub mod error;
pub use error::WindowError;

// GLFW - 'Context' trait needed for the 'create_window()' function
use glfw::Context;

type Result<T> = std::result::Result<T, error::WindowError>;

// Default window size
const DEFAULT_WINDOW_WIDTH: u32 = 512;
const DEFAULT_WINDOW_HEIGHT: u32 = 512;

// Default Window title
const DEFAULT_WINDOW_TITLE: &str = "OpenGL";

// Default OpenGL API Version
const GL_MAJOR_VERSION: u32 = 4;
const GL_MINOR_VERSION: u32 = 6;

// Default Window Mode
const DEFAULT_WINDOW_MODE: glfw::WindowMode = glfw::WindowMode::Windowed;

// Type-Alias for readability
type GLFW = glfw::Glfw;
type Window = glfw::PWindow;
type WindowEvents = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

// Main struct
#[derive(Debug)]
pub struct GLWindow {
    pub(crate) glfw: GLFW,
    pub(crate) window: Window,
    pub(crate) events: WindowEvents,
}

impl GLWindow {
    // Creates and opens a new window, with specified dimensions, API version, and title
    // Returns a handle to the window, and to the window's event loop
    pub fn new<T: AsRef<str>>(
        title: T,
        width: u32,
        height: u32,
        gl_major_version: u32,
        gl_minor_version: u32,
    ) -> Result<GLWindow> {
        glfw::init_no_callbacks()
            .and_then(|mut glfw| {
                // Set the version of OpenGL we're using
                glfw.window_hint(glfw::WindowHint::ContextVersion(
                    gl_major_version,
                    gl_minor_version,
                ));

                // Load the only the core, i.e. no extended, protocols
                glfw.window_hint(glfw::WindowHint::OpenGlProfile(
                    glfw::OpenGlProfileHint::Core,
                ));

                // Don't allow use of deprecated features
                glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
                Ok(glfw)
            })
            .map_err(|glfw_init_error| WindowError::FailedToInitializeGLFW(glfw_init_error))
            .and_then(|mut glfw| {
                glfw.create_window(width, height, title.as_ref(), DEFAULT_WINDOW_MODE)
                    .ok_or(WindowError::FailedToCreateWindow)
                    .and_then(|(window, events)| Ok((glfw, window, events)))
            })
            .and_then(|(glfw, mut window, events)| {
                // This function makes the OpenGL or OpenGL ES context of the specified window current on the calling thread
                window.make_current();
                // Notify us when a keyboard button is pressed
                window.set_key_polling(true);
                // Notify us when the window size (and therefore the frame buffer) changes
                window.set_framebuffer_size_polling(true);
                Ok(GLWindow {
                    glfw,
                    window,
                    events,
                })
            })
    }

    // Convenience function to open a standard sized window
    pub fn default() -> Result<GLWindow> {
        Self::new(
            DEFAULT_WINDOW_TITLE,
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
            GL_MAJOR_VERSION,
            GL_MINOR_VERSION,
        )
    }
}
