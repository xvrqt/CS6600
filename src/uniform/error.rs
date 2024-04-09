// Error type for Uniforms
#[derive(Debug)]
pub enum UniformError {
    VectorLength,
    MatrixConversion((u8, u8)),
    SettingUniformValue(String),
    GetUniformLocation(String),
}

impl std::error::Error for UniformError {}
impl std::fmt::Display for UniformError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UniformError::VectorLength => {
                write!(
                    f,
                    "Failed to convert length of uniform vector from 'usize' to 'GLsizei'.",
                )
            }
            UniformError::MatrixConversion((a, b)) => {
                write!(
                    f,
                    "Could not convert Vec<f32> is not a multiple of {}x{}",
                    a, b
                )
            }
            UniformError::SettingUniformValue(error) => {
                write!(f, "Failed to set Uniform Value.\n{}", error)
            }
            UniformError::GetUniformLocation(name) => {
                write!(f, "Failed to find the location for '{}'.\n", name)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<UniformError> for crate::GLError {
    fn from(error: UniformError) -> Self {
        crate::GLError::Uniform(error)
    }
}
// Allows for painless casting into our crate's rollup error
impl From<UniformError> for crate::program::ProgramError {
    fn from(error: UniformError) -> Self {
        crate::program::ProgramError::Uniform(error)
    }
}
