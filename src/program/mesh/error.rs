use crate::GLUtilityError;

// Error type for source loading
#[derive(Debug)]
pub enum MeshError {
    FailedToParseFile(String),
    UnknownFileType(String),
    Other(GLUtilityError),
}

impl std::error::Error for MeshError {}
impl std::fmt::Display for MeshError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MeshError::FailedToParseFile(path) => {
                write!(f, "Could not parse file '{}' into a Mesh", path)
            }
            MeshError::UnknownFileType(extension) => {
                write!(f, "Did not recognize 3D file type: '.{}'", extension)
            }
            MeshError::Other(error) => {
                write!(f, "Encountered a Mesh Error: {}\n", error)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<MeshError> for crate::GLError {
    fn from(error: MeshError) -> Self {
        crate::GLError::Mesh(error)
    }
}

// Allows for painless casting
impl From<std::io::Error> for MeshError {
    fn from(error: std::io::Error) -> Self {
        let glu_error: GLUtilityError = error.into();
        MeshError::Other(glu_error)
    }
}

// Different Parser Error Conversions
// Wavefront Objects (.obj)
impl From<wavefront_obj::ParseError> for MeshError {
    fn from(error: wavefront_obj::ParseError) -> Self {
        MeshError::FailedToParseFile(error.to_string())
    }
}
