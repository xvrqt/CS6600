use crate::vao::VAOError;
use crate::GLError;
use crate::GLUtilityError;
use gl::types::*;

// OpenGL attribute handle and corresponding buffer handle
#[derive(Debug)]
pub struct Attribute {
    // Handle to the attribute location in the shader
    pub id: GLuint,
    // Handle to the backing buffer of the attribute
    pub buffer: GLuint,
}

// Data types that implement this trait define how to set their OpenGL
// attribute pointers
impl Attribute {
    pub fn new<S>(program_id: GLuint, name: S) -> Result<Self, VAOError>
    where
        S: AsRef<str>,
    {
        let id;
        let buffer;
        unsafe {
            // Get the handle to where the attribute is located in the shader
            let name = std::ffi::CString::new(name.as_ref().as_bytes())
                .map_err(|_| VAOError::FailedIDConversion)?;
            let attribute_location = gl::GetAttribLocation(program_id, name.as_ptr());
            if attribute_location < 0 {
                // TODO: Make a proper error type
                return Err(VAOError::CouldNotFindLocation("gay".to_string()));
            }

            // Generate a buffer that we can write to
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            buffer = vbo;

            // Convert to GLuint from GLint
            id = attribute_location
                .try_into()
                .map_err(|_| VAOError::FailedIDConversion)?;
        }
        Ok(Attribute { id, buffer })
    }
}
