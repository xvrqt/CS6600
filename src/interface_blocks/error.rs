use crate::GLUtilityError;
use std::ffi::CString;

// Error type for Uniforms
#[derive(Debug)]
pub enum InterfaceBlockError {
    VectorLength,
    MatrixConversion((u8, u8)),
    SettingUniformValue(String),
    CouldNotFindUniformIndex(CString),
    Other(GLUtilityError),
}

impl std::error::Error for InterfaceBlockError {}
impl std::fmt::Display for InterfaceBlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterfaceBlockError::VectorLength => {
                write!(
                    f,
                    "Failed to convert length of uniform vector from 'usize' to 'GLsizei'.",
                )
            }
            InterfaceBlockError::MatrixConversion((a, b)) => {
                write!(
                    f,
                    "Could not convert Vec<f32> is not a multiple of {}x{}",
                    a, b
                )
            }
            InterfaceBlockError::SettingUniformValue(error) => {
                write!(f, "Failed to set Uniform Value.\n{}", error)
            }
            InterfaceBlockError::CouldNotFindUniformIndex(name) => {
                write!(
                    f,
                    "Failed to find the location for '{}'.\n",
                    name.to_string_lossy()
                )
            }
            InterfaceBlockError::Other(error) => {
                write!(f, "{}", error)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<InterfaceBlockError> for crate::GLError {
    fn from(error: InterfaceBlockError) -> Self {
        crate::GLError::InterfaceBlock(error)
    }
}
// Allows for painless casting into our crate's rollup error
impl From<InterfaceBlockError> for crate::program::ProgramError {
    fn from(error: InterfaceBlockError) -> Self {
        crate::program::ProgramError::InterfaceBlock(error)
    }
}
