#[derive(Debug)]
pub enum WindowError {
    GLFWInitFailed,
    WindowCreateFailed,
}

impl std::error::Error for WindowError {}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WindowError::GLFWInitFailed => {
                write!(f, "Failed to initialize GLFW context.\n")
            }
            WindowError::WindowCreateFailed => write!(f, "Failed to create a window."),
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<WindowError> for crate::GLError {
    fn from(error: WindowError) -> Self {
        crate::GLError::Window(error)
    }
}
