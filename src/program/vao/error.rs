// Error type for Vertex Array Objects
#[derive(Debug)]
pub enum VAOError {
    VectorLength,
    SetAttributePointer,
    AttributeAlreadyExists(String),
    FailedToGetActiveProgram,
    CStringConversion(String),
    FailedIDConversion,
    CouldNotFindLocation(String),
    CouldNotFindAttribute(String),
}

impl std::error::Error for VAOError {}
impl std::fmt::Display for VAOError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VAOError::VectorLength => {
                write!(
                    f,
                    "Failed to convert length of uniform vector from 'usize' to 'GLsizei'.",
                )
            }
            VAOError::CStringConversion(name) => {
                write!(f, "Could not convert string: '{}' to c string", name,)
            }
            VAOError::FailedIDConversion => {
                write!(f, "Could not convert IDs from GLint to GLuint",)
            }
            VAOError::FailedToGetActiveProgram => {
                write!(f, "Could not get or convert the active GL program",)
            }
            VAOError::SetAttributePointer => {
                write!(f, "Could not convert Vec<f32> is not a multiple",)
            }
            VAOError::CouldNotFindLocation(name) => {
                write!(f, "Attribute: {} could not be found in the shader.", name)
            }
            VAOError::AttributeAlreadyExists(name) => {
                write!(f, "Attribute: {} already exists for this VAO.", name)
            }
            VAOError::CouldNotFindAttribute(name) => {
                write!(f, "Could not find attribute: {}", name)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<VAOError> for crate::GLError {
    fn from(error: VAOError) -> Self {
        crate::GLError::VAO(error)
    }
}

// Allows for painless casting into our crate's rollup error
// VAOs only exist in the context of Programs
impl From<VAOError> for crate::program::ProgramError {
    fn from(error: VAOError) -> Self {
        crate::program::ProgramError::VAO(error)
    }
}
