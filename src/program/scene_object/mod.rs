use crate::program::mesh::Mesh;
use crate::program::GLDraw;

pub mod error;
pub use error::SceneObjectError;
type Result<T> = std::result::Result<T, SceneObjectError>;
use super::mesh::ATTACHED;

#[derive(Debug)]
pub(crate) struct SceneObject {
    enabled: bool,
    mesh: Mesh<ATTACHED>,
}

impl SceneObject {
    pub(crate) fn new(mesh: Mesh<ATTACHED>) -> Self {
        SceneObject {
            enabled: true,
            mesh,
        }
    }
}

impl GLDraw for SceneObject {
    fn draw(&mut self) -> super::Result<()> {
        if self.enabled {
            self.mesh.draw()
        } else {
            Ok(())
        }
    }
}
