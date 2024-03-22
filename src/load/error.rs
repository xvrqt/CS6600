// Error type for source loading
#[derive(Debug)]
pub enum LoadError {
    FailedToLoadShaderSource(String),
}

impl std::error::Error for LoadError {}
impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadError::FailedToLoadShaderSource(error) => {
                write!(f, "Failed to load shader source.\n{}", error)
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
