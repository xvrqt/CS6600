// Opening a window with an OpenGL conteext
pub mod error;
pub use error::{GLError, GLUtilityError};
pub mod window;

// Linking shaders to crete a GL Program
pub mod program;
pub use program::builder;
pub use program::GLProgram;

// Compiling shaders into OpenGL programs
pub mod shader;
pub use shader::Shader;
// Loading shader, object, texture files
pub mod load;
// Creating and managing Vertex Array Objects
pub mod vao;
// Types and Setting Uniform Values
pub mod uniform;
// Our Wavefront Obj parses into this
pub mod obj;

// Re-export for more ergnomic use
pub mod types {
    pub use crate::uniform::*;
}
