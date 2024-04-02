// Library Error Types
pub use crate::{
    load::LoadError, program::ProgramError, shader::ShaderError, uniform::UniformError,
    vao::VAOError, window::WindowError,
};
// Make error logs, and shader source errors pretty and helpful
use bat::PrettyPrinter;

// Our Errors will all roll up into this error type for easy handling
#[derive(Debug)]
pub enum GLError {
    Program(ProgramError),
    Shader(ShaderError),
    Window(WindowError),
    Uniform(UniformError),
    VAO(VAOError),
    Load(LoadError),
    Other(GLUtilityError),
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
            GLError::Other(error) => {
                write!(f, "GL Program Error:\n{}", error.to_string())
            }
        }
    }
}

// Some errors are similar across stucturs, such as failing to find an index, or converting into a
// CString or into a pointer. Standardizing the messages using these errors.
#[derive(Debug)]
pub enum GLUtilityError {
    FailedToConvertToCString(String),
    ErrorLog(String),
    CouldNotCreateErrorLog,
    CouldNotOpenFile(String, std::io::Error),
}

impl std::error::Error for GLUtilityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CouldNotOpenFile(_, io_error) => Some(io_error),
            _ => None,
        }
    }
}
impl std::fmt::Display for GLUtilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GLUtilityError::FailedToConvertToCString(source_string) => {
                let s = &source_string[0..15];
                write!(f, "Failed to convert the string: \"{}...\" to a CString", s)
            }
            GLUtilityError::CouldNotOpenFile(path, error) => {
                write!(f, "Failed to open the file at path: \"{}\"", path)
            }
            GLUtilityError::ErrorLog(log) => {
                let mut pp = PrettyPrinter::new();
                pp.input_from_bytes(log.as_bytes());
                pp.language("glsl");

                if let Err(_) = pp.print() {
                    write!(f, "{}", log)
                } else {
                    write!(f, "")
                }
            }
            GLUtilityError::CouldNotCreateErrorLog => {
                write!(
                    f,
                    "Error encountered, but could not create the error log :["
                )
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<GLUtilityError> for crate::GLError {
    fn from(error: GLUtilityError) -> Self {
        crate::GLError::Other(error)
    }
}
