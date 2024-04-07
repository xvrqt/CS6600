// Converts various 3D file types into our internal represntation 'Mesh'
mod conversions;
mod error;
pub use conversions::load_mesh;
pub use error::MeshError;

// Linear algebra types we use in our internal representation
use ultraviolet::vec::{Vec2, Vec3};

// Standard internal format for 3D models, including vertex positions, per-vertex normals, and
// per-vertex ST coordinates.
#[derive(Debug)]
pub struct Mesh {
    pub(crate) vertices: Vec<Vec3>,
    pub(crate) normals: Vec<Vec3>,
    pub(crate) st_coordinates: Vec<Vec2>,
    pub(crate) indices: Vec<u32>,
}
