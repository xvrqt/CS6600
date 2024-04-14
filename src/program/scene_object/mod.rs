// Error Types
pub mod error;
pub use error::SceneObjectError;
type Result<T> = std::result::Result<T, SceneObjectError>;

// Linear Algebra Types
use ultraviolet::mat::{Mat3, Mat4};

// If the option is set, then the SceneObject has updated its transformation. If it's None then it
// does not need to be updated
#[derive(Debug)]
pub(crate) struct SceneObject {
    pub(crate) enabled: bool,
    model_transform: Mat4,
    world_transform: Mat4,
    // Pre-multiplied: world * model transforms. None if it hasn't changed since last query.
    pub(crate) transform: Option<Mat4>,
    pub(crate) normal_transform: Option<Mat3>, // Might not need be an option
}

impl SceneObject {
    pub(crate) fn new(model_transform: Mat4) -> Self {
        let normal_transform = model_transform.inversed().transposed().truncate();
        let normal_transform = Some(normal_transform);
        SceneObject {
            enabled: true,
            model_transform,
            world_transform: Mat4::identity(),
            transform: Some(model_transform),
            normal_transform,
        }
    }
}

impl Default for SceneObject {
    fn default() -> Self {
        SceneObject {
            enabled: true,
            model_transform: Mat4::identity(),
            world_transform: Mat4::identity(),
            transform: Some(Mat4::identity()),
            normal_transform: Some(Mat3::identity()),
        }
    }
}
