use crate::program::vao::{VAOError, VAO};
use crate::program::ProgramError;
use crate::GLUtilityError;

// Error type for source loading
#[derive(Debug)]
pub enum SceneObjectError {
    FailedToParseFile(String),
    UnknownFileType(String),
    Other(GLUtilityError),
    VAO(VAOError),
}

impl std::error::Error for SceneObjectError {}
impl std::fmt::Display for SceneObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SceneObjectError::FailedToParseFile(path) => {
                write!(f, "Could not parse file '{}' into a Mesh", path)
            }
            SceneObjectError::UnknownFileType(extension) => {
                write!(f, "Did not recognize 3D file type: '.{}'", extension)
            }
            SceneObjectError::Other(error) => {
                write!(f, "Encountered a Mesh Error: {}\n", error)
            }
            SceneObjectError::VAO(error) => {
                write!(f, "Encountered a VAO Error: {}\n", error)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<SceneObjectError> for crate::GLError {
    fn from(error: SceneObjectError) -> Self {
        crate::GLError::SceneObject(error)
    }
}

impl From<SceneObjectError> for ProgramError {
    fn from(error: SceneObjectError) -> Self {
        ProgramError::SceneObject(error)
    }
}

impl From<VAOError> for SceneObjectError {
    fn from(error: VAOError) -> Self {
        SceneObjectError::VAO(error)
    }
}

// Allows for painless casting
impl From<std::io::Error> for SceneObjectError {
    fn from(error: std::io::Error) -> Self {
        let glu_error: GLUtilityError = error.into();
        SceneObjectError::Other(glu_error)
    }
}

// Different Parser Error Conversions
// Wavefront Objects (.obj)
impl From<wavefront_obj::ParseError> for SceneObjectError {
    fn from(error: wavefront_obj::ParseError) -> Self {
        SceneObjectError::FailedToParseFile(error.to_string())
    }
}
