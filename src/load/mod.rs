pub mod error;
pub use error::LoadError;

use crate::obj::Obj;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use obj;

// Loads a Wavefront .obj File
// Extracts the vertices, normals, and UV coordinates
// Provides an implementation of Attribute that sets these up on a VAO
// It will use the DrawElements strategy of rendering
pub fn load_obj<P>(path: P) -> Result<Obj, LoadError>
where
    P: AsRef<Path>,
{
    let file = File::open(path.as_ref())?;
    let input = BufReader::new(file);
    // We plan to import positions, normals, and UV coordinates
    let obj: obj::Obj<obj::TexturedVertex, u16> = obj::load_obj(input)?;
    Ok(obj.into())
}

// Loads a shader from a path, nothing special
pub fn load_shader<P>(path: P) -> Result<String, LoadError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    fs::read_to_string(path).map_err(|e| LoadError::FailedToLoadShaderSource(e.to_string()))
}
