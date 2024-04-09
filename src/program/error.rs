use super::mesh::MeshError;
use crate::program::vao::VAOError;
use crate::shader::error::ShaderError;
use crate::window::WindowError;

// Error for GLProgram
#[derive(Debug)]
pub enum ProgramError {
    ShaderCompilation(ShaderError),
    Linking(String),
    SettingUniformValue(String),
    GetUniformLocation(String),
    VAOAlreadyExists(String),
    VAODoesNotExist(String),
    VAO(VAOError),
    Window(WindowError),
    Mesh(MeshError),
    End,
}

impl std::error::Error for ProgramError {}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProgramError::ShaderCompilation(shader_error) => {
                write!(f, "Error in processing a shader.\n{}", shader_error)
            }
            ProgramError::Linking(error_log) => {
                write!(f, "Could not link shaders to the program.\n{}", error_log)
            }
            ProgramError::SettingUniformValue(error) => {
                write!(f, "Failed to set Uniform Value.\n{}", error)
            }
            ProgramError::GetUniformLocation(name) => {
                write!(f, "Failed to find the location for '{}'.\n", name)
            }
            ProgramError::VAO(error) => {
                write!(f, "VAO ERROR: '{}'.\n", error)
            }
            ProgramError::End => {
                write!(f, "Window was closed")
            }
            ProgramError::Window(error) => {
                write!(f, "Window ERROR: '{}'.\n", error)
            }
            ProgramError::Mesh(error) => {
                write!(f, "Mesh ERROR: '{}'.\n", error)
            }
            ProgramError::VAOAlreadyExists(name) => {
                write!(
                    f,
                    "VAO with name: '{}' already exists for this program.\n",
                    name
                )
            }
            ProgramError::VAODoesNotExist(name) => {
                write!(
                    f,
                    "VAO with name: '{}' does not exist for this program. Cannot set attribute on it.\n",
                    name
                )
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<ProgramError> for crate::GLError {
    fn from(error: ProgramError) -> Self {
        crate::GLError::Program(error)
    }
}
