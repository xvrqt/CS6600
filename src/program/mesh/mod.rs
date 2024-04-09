// Converts various 3D file types into our internal represntation 'Mesh'
mod conversions;
use super::vao::VAO;
pub use crate::program::Attribute;

// Linear algebra types we use in our internal representation
use gl::types::*;
use ultraviolet::vec::{Vec2, Vec3};

// Error Types
mod error;
pub use error::MeshError;
type Result<T> = std::result::Result<T, MeshError>;

// Standard Library
use std::path::Path;

#[derive(Debug)]
pub enum MeshState {
    Unloaded,
    Loaded(GLuint, GLuint, GLuint),
}

pub struct UNATTACHED {}
pub struct ATTACHED {
    pub(crate) vao: VAO,
}

// Standard internal format for 3D models, including vertex positions, per-vertex normals, and
// per-vertex ST coordinates.
#[derive(Debug)]
pub struct Mesh<State> {
    pub(crate) vertices: Vec<Vec3>,
    pub(crate) normals: Vec<Vec3>,
    pub(crate) st_coordinates: Vec<Vec2>,
    pub(crate) indices: Vec<u32>,
    pub(crate) program_data: State,
}

// Meshes can be created outside of a GLProgram (to save overhead in reading from disk and parsing)
// and attached to multiple GLPrograms. GLPrograms must `attach()` meshes so that they generate a
// Vertex Attrib Object that can be used to render them. They cannot do this without knowing which
// GLProgram they are attaching to.
impl Mesh<UNATTACHED> {
    // Creates a VAO from the mesh, and loades the arrays to the GPU
    pub(crate) fn attach(self, program_id: GLuint) -> Result<Mesh<ATTACHED>> {
        // Move everything except `program_data`
        let Mesh {
            vertices,
            normals,
            st_coordinates,
            indices,
            ..
        } = self;

        // Attach these buffers as attributes when creating our VAO
        let attributes = vec![
            ("vertices".to_string(), &vertices),
            ("normals".to_string(), &normals),
        ];
        let vao = VAO::new(program_id, &indices, Some(&attributes))?;
        let program_data = ATTACHED { vao };

        Ok(Mesh {
            vertices,
            normals,
            st_coordinates,
            indices,
            program_data,
        })
    }

    // Load a mesh from a Path
    pub fn parse<P>(path: P) -> Result<Mesh<UNATTACHED>>
    where
        P: AsRef<Path>,
    {
        conversions::load_mesh(path)
    }

    // Creates a Mesh from various types returned by varios 3D file parser libraries
    pub fn from_obj(obj: wavefront_obj::obj::Object) -> Mesh<UNATTACHED> {
        obj.into()
    }
}
