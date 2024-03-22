pub mod error;
pub use error::LoadError;

use std::fs;
use std::path::Path;

pub fn load_shader<P>(path: P) -> Result<String, LoadError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    fs::read_to_string(path).map_err(|e| LoadError::FailedToLoadShaderSource(e.to_string()))
}
