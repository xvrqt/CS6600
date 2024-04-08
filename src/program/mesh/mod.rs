// Converts various 3D file types into our internal represntation 'Mesh'
mod conversions;
mod error;
pub use crate::program::Attribute;
pub use conversions::load_mesh;
pub use error::MeshError;

// Linear algebra types we use in our internal representation
use gl::types::*;
use ultraviolet::vec::{Vec2, Vec3};

use crate::vao::VAO;

// Convenience Error Type
type Result<T> = std::result::Result<T, MeshError>;

#[derive(Debug)]
pub enum MeshState {
    Unloaded,
    Loaded(GLuint, GLuint, GLuint),
}

pub struct UNLOADED {}
pub struct LOADED(pub VAO);

// Standard internal format for 3D models, including vertex positions, per-vertex normals, and
// per-vertex ST coordinates.
#[derive(Debug)]
pub struct Mesh<State> {
    pub(crate) vertices: Vec<Vec3>,
    pub(crate) normals: Vec<Vec3>,
    pub(crate) st_coordinates: Vec<Vec2>,
    pub(crate) indices: Vec<u32>,
    pub(crate) state: State,
}

impl Mesh<UNLOADED> {
    // Creates a VAO from the mesh, and loades the arrays to the GPU
    pub(crate) fn load(self, program_id: GLuint) -> Result<Mesh<LOADED>> {
        let mut state = LOADED(VAO::new(program_id, self.indices.clone()));
        let Mesh {
            vertices,
            normals,
            st_coordinates,
            indices,
            ..
        } = self;
        state.0.attribute("vertices", vertices.clone())?;
        state.0.attribute("normals", normals.clone())?;
        state.0.draw_count = indices.len() as i32;
        Ok(Mesh {
            vertices,
            normals,
            st_coordinates,
            indices,
            state,
        })
    }
}
