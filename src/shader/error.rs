// Error type for Shaders
#[derive(Debug)]
pub enum ShaderError {
    SourceParse,
    SourceCompilation(String),
    UnknownType,
}

impl std::error::Error for ShaderError {}
impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShaderError::SourceParse => {
                write!(f, "Failed to convert the shader's source into a c-string.")
            }
            ShaderError::SourceCompilation(error_log) => write!(
                f,
                "Failed to compile the shader from source.\n{}",
                error_log
            ),
            ShaderError::UnknownType => write!(
                f,
                "Could not determine shader type. How did you even get this error. This is a library error.",
            ),
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<ShaderError> for crate::GLError {
    fn from(error: ShaderError) -> Self {
        crate::GLError::Shader(error)
    }
}

// Allows for painless casting into our crate's rollup error
impl From<ShaderError> for crate::program::error::ProgramError {
    fn from(error: ShaderError) -> Self {
        crate::program::error::ProgramError::ShaderCompilation(error)
    }
}
