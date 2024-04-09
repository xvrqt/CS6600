use crate::program::mesh::Mesh;
use crate::program::GLDraw;
use crate::uniform::GayUniform;
use gl::types::*;

pub mod error;
pub use error::SceneObjectError;
type Result<T> = std::result::Result<T, SceneObjectError>;
use super::mesh::ATTACHED;
use crate::uniform::Uniform;
use std::rc::Rc;
use ultraviolet::mat::Mat4;

#[derive(Debug)]
pub(crate) struct SceneObject {
    program_id: GLuint,
    enabled: bool,
    mesh: Rc<Mesh<ATTACHED>>,
    pub(crate) object_transform: Mat4,
}

impl SceneObject {
    pub(crate) fn new(
        program_id: GLuint,
        mesh: Rc<Mesh<ATTACHED>>,
        object_transform: Mat4,
    ) -> Self {
        SceneObject {
            program_id,
            enabled: true,
            mesh,
            object_transform,
        }
    }
}

impl GLDraw for SceneObject {
    fn draw(&self) -> super::Result<()> {
        if self.enabled {
            GayUniform::set_uniform(self.program_id, "object_transform", self.object_transform)?;
            self.mesh.draw()
        } else {
            Ok(())
        }
    }
}
