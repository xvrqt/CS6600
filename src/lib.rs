// Opening a window with an OpenGL conteext
pub mod error;
pub use error::{GLError, GLStatus, GLUtilityError};
pub mod window;
pub use program::lights::{LightColor, Position};

// Linking shaders to crete a GL Program
pub mod program;
pub use program::builder;
pub use program::GLProgram;

// Compiling shaders into OpenGL programs
pub mod shader;
pub use shader::Shader;

pub mod materials;
pub use materials::Material;
// Loading shader, object, texture files
// Creating and managing Vertex Array Objects
// Types and Setting Uniform Values
pub use program::mesh::Mesh;
pub mod interface_blocks;
pub mod uniform;
pub use uniform::Uniform;
// Our Wavefront Obj parses into this

// Re-export for more ergnomic use
pub mod types {
    pub use crate::uniform::*;
}
