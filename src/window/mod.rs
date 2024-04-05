use std::collections::VecDeque;

// Import our Error Type
pub use crate::program::camera::CameraEvent;
use crate::{program::camera::Direction, GLError};
use glfw::{Action, Key};
use ultraviolet::vec::Vec3;
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
type WindowGLFWEvents = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

// Main struct
#[derive(Debug)]
pub struct GLWindow {
    pub(crate) glfw: GLFW,
    pub(crate) window: Window,
    pub(crate) events: WindowGLFWEvents,
    pub(crate) frame_state: FrameState,
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
                // Notify us of certain events
                window.set_scroll_polling(true);
                window.set_cursor_pos_polling(true);
                window.set_mouse_button_polling(true);
                window.set_framebuffer_size_polling(true);
                let frame_state = FrameState::new(&glfw);
                Ok(GLWindow {
                    glfw,
                    window,
                    events,
                    frame_state,
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

    // Used in the render loop to set the FrameState
    pub fn process_events(&mut self) -> () {
        // Get Updated Time
        let current_time = self.glfw.get_time() as f32;
        let dt = current_time - self.frame_state.time;
        self.frame_state.time = current_time;
        self.frame_state.delta_t = dt;

        // Clear Event Queues
        self.frame_state.window_events.clear();

        // Retrieve and normalize cursor coordinates
        let (width, height) = self.window.get_size();
        let aspect_ratio = width as f32 / height as f32;
        let (x, y) = self.window.get_cursor_pos();
        let u = (((x as f32 / width as f32) * 2.0) - 1.0) * aspect_ratio;
        let v = -(((y as f32 / height as f32) * 2.0) - 1.0);

        // Let the camera know the middle mouse drag
        if self.frame_state.mm_shift_valid {
            let (ou, ov) = self.frame_state.mm_cursor_position;
            let x = u - ou;
            let y = v - ov;
            // println!("x: {}, y: {}", x, y);
            self.frame_state
                .camera_events
                .push_back(CameraEvent::Movement(Direction::Vector(x, y, 0.0)));
            self.frame_state.mm_cursor_position = (u, v);
        }

        // Let the camera know the middle mouse drag
        if self.frame_state.mm_valid {
            let (ou, ov) = self.frame_state.mm_cursor_position;
            let x = u - ou;
            let y = v - ov;
            // println!("x: {}, y: {}", x, y);
            self.frame_state
                .camera_events
                .push_back(CameraEvent::Rotation(Direction::Vector(x, y, 0.0)));
            self.frame_state.mm_cursor_position = (u, v);
        }

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                // Update Viewport, and Resolution Shader Uniform
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    self.frame_state.resolution = Some((width as f32, height as f32));
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::ProjectionAspectRatio(aspect_ratio));
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                    self.frame_state.window_events.push(WindowEvent::Close);
                }
                glfw::WindowEvent::Key(Key::P, _, Action::Press, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::SwapProjection);
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        // .push_back(CameraEvent::Movement(Direction::Forwards(dt)));
                        .push_back(CameraEvent::Rotation(Direction::Up));
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::Rotation(Direction::Down));
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::Rotation(Direction::Left(dt)));
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::Rotation(Direction::Right(dt)));
                }
                glfw::WindowEvent::Key(Key::Z, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::ZoomProjection(0.1));
                }
                glfw::WindowEvent::Key(Key::X, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::ZoomProjection(-0.1));
                }
                glfw::WindowEvent::MouseButton(
                    glfw::MouseButtonMiddle,
                    Action::Press,
                    glfw::Modifiers::Shift,
                ) => {
                    self.frame_state.mm_shift_cursor_position = (u, v);
                    self.frame_state.mm_shift_valid = true;
                }
                glfw::WindowEvent::MouseButton(
                    glfw::MouseButtonMiddle,
                    Action::Release,
                    glfw::Modifiers::Shift,
                ) => {
                    self.frame_state.mm_shift_valid = false;
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButtonMiddle, Action::Press, _) => {
                    self.frame_state.mm_cursor_position = (u, v);
                    self.frame_state.mm_valid = true;
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButtonMiddle, Action::Release, _) => {
                    self.frame_state.mm_valid = false;
                }
                glfw::WindowEvent::Key(Key::C, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::Movement(Direction::Center));
                }
                glfw::WindowEvent::Key(Key::F, _, Action::Press | Action::Repeat, _) => {
                    self.frame_state
                        .camera_events
                        .push_back(CameraEvent::Movement(Direction::Flip));
                }
                glfw::WindowEvent::Scroll(_, y) => {
                    let y = y as f32;
                    if y > 0.0 {
                        self.frame_state
                            .camera_events
                            .push_back(CameraEvent::Movement(Direction::Forwards(dt)));

                        // .push_back(CameraEvent::Projection(Direction::In(y)))
                    } else {
                        self.frame_state
                            .camera_events
                            .push_back(CameraEvent::Movement(Direction::Backwards(dt)));
                        // .push_back(CameraEvent::Projection(Direction::Out(y)))
                    }
                }
                _ => {}
            }
        }
    }
}

// This is used by GLPrograms to update their magic variables
#[derive(Debug)]
pub struct FrameState {
    pub time: f32,                      // Total time elapsed
    pub delta_t: f32,                   // Time since the previous frame
    pub resolution: Option<(f32, f32)>, // Width, Height
    pub toggle_projection: bool,
    pub mm_cursor_position: (f32, f32),
    pub mm_shift_cursor_position: (f32, f32),
    pub mm_valid: bool,
    pub mm_shift_valid: bool,
    pub camera_events: std::collections::VecDeque<CameraEvent>,
    pub window_events: std::vec::Vec<WindowEvent>,
}

impl FrameState {
    fn new(glfw: &glfw::Glfw) -> FrameState {
        FrameState {
            time: glfw.get_time() as f32,
            delta_t: 0.0,
            resolution: None, // Only contains Some() when the screen changes size to avoid needless recalculations
            toggle_projection: false,
            mm_cursor_position: (0.0, 0.0),
            mm_shift_cursor_position: (0.0, 0.0),
            mm_valid: false,
            mm_shift_valid: false,
            camera_events: VecDeque::new(),
            window_events: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum WindowEvent {
    Close,
}
