#[derive(Debug)]
pub enum WindowError {
    FailedToInitializeGLFW(glfw::InitError),
    FailedToCreateWindow,
}

impl std::error::Error for WindowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WindowError::FailedToInitializeGLFW(glfw_init_error) => Some(glfw_init_error),
            _ => None,
        }
    }
}
impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WindowError::FailedToInitializeGLFW(_) => {
                write!(f, "Failed to initialize the GLFW context.")
            }
            WindowError::FailedToCreateWindow => {
                write!(f, "Failed to create a window with an OpenGL context.")
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<WindowError> for crate::GLError {
    fn from(error: WindowError) -> Self {
        crate::GLError::Window(error)
    }
}

// Allows for painless casting into our crate's rollup error
impl From<WindowError> for crate::program::ProgramError {
    fn from(error: WindowError) -> Self {
        crate::program::ProgramError::Window(error)
    }
}
