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
    trans_index: usize,
}

impl SceneObject {
    pub(crate) fn new(program_id: GLuint, mesh: Rc<Mesh<ATTACHED>>, trans_index: usize) -> Self {
        SceneObject {
            program_id,
            enabled: true,
            mesh,
            trans_index,
        }
    }
}

impl GLDraw for SceneObject {
    fn draw(&self) -> super::Result<()> {
        if self.enabled {
            // println!("setting object_transform_index to: {}", self.trans_index);
            GayUniform::set_uniform(
                self.program_id,
                "object_transform_index",
                &crate::uniform::GL1U(self.trans_index as u32),
            )?;
            self.mesh.draw()
        } else {
            Ok(())
        }
    }
}
