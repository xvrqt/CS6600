use obj::ObjError;

// Error type for source loading
#[derive(Debug)]
pub enum LoadError {
    FailedToLoadShaderSource(String),
    FailedToLoadObjFromSource(String),
    FailedToParseObj(String),
}

impl std::error::Error for LoadError {}
impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadError::FailedToLoadShaderSource(error) => {
                write!(f, "Failed to load shader source.\n{}", error)
            }
            LoadError::FailedToLoadObjFromSource(error) => {
                write!(f, "Failed to load .obj source.\n{}", error)
            }
            LoadError::FailedToParseObj(error) => {
                write!(f, "Failed to parse .obj source.\n{}", error)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<LoadError> for crate::GLError {
    fn from(error: LoadError) -> Self {
        crate::GLError::Load(error)
    }
}

// Allows for painless casting
impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        LoadError::FailedToLoadObjFromSource(error.to_string())
    }
}

// Allows for painless casting
impl From<ObjError> for LoadError {
    fn from(error: ObjError) -> Self {
        LoadError::FailedToParseObj(error.to_string())
    }
}
