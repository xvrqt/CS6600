// Error Types
pub mod error;
pub use error::SceneObjectError;
type Result<T> = std::result::Result<T, SceneObjectError>;

// Linear Algebra Types
use ultraviolet::mat::Mat4;

// If the option is set, then the SceneObject has updated its transformation. If it's None then it
// does not need to be updated
#[derive(Debug)]
pub(crate) struct SceneObject {
    pub(crate) enabled: bool,
    pub(crate) transform: Option<Mat4>,
}

impl SceneObject {
    pub(crate) fn new(transform: Mat4) -> Self {
        SceneObject {
            enabled: true,
            transform: Some(transform),
        }
    }
}

impl Default for SceneObject {
    fn default() -> Self {
        SceneObject {
            enabled: true,
            transform: Some(Mat4::identity()),
        }
    }
}
