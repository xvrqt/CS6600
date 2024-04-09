// Converts various 3D file types into our internal represntation 'Mesh'
mod conversions;
use super::vao::VAO;
use super::GLDraw;
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

#[derive(Debug, Clone)]
pub struct UNATTACHED {}
#[derive(Debug)]
pub struct ATTACHED {
    pub(crate) vao: VAO,
}

// Standard internal format for 3D models, including vertex positions, per-vertex normals, and
// per-vertex ST coordinates.
#[derive(Debug, Clone)]
pub struct Mesh<State> {
    pub(crate) vertices: Vec<Vec3>,
    pub(crate) normals: Vec<Vec3>,
    pub(crate) st_coordinates: Vec<Vec2>,
    pub(crate) indices: Vec<u32>,
    pub(crate) program_data: State,
    pub(crate) draw_style: GLuint,
}

impl GLDraw for Mesh<ATTACHED> {
    fn draw(&mut self) -> super::Result<()> {
        let vao = &self.program_data.vao;
        unsafe {
            gl::BindVertexArray(vao.id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vao.elements.buffer_id);
            gl::DrawElements(
                self.draw_style,
                vao.elements.buffer_length,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
        Ok(())
    }
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
            draw_style,
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
            draw_style,
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

    // Updates the draw style
    // TODO: Wrap in ENUMs so we don't need to expose gl::consts
    pub fn draw_style(&mut self, draw_style: GLuint) -> () {
        self.draw_style = draw_style;
    }
}
