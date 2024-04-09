use crate::program::vao::VAOError;
use crate::types::*;
use gl::types::*;
use std::ptr;
use ultraviolet::vec::*;
type Result<T> = std::result::Result<T, VAOError>;

// OpenGL attribute handle and corresponding buffer handle
#[derive(Debug)]
pub struct Attribute {
    // Attribute location in the shader
    pub location: GLuint,
    // ID of the backing OpenGL buffer of the attribute
    pub buffer_id: GLuint,
}

// Data types that implement this trait define how to set their OpenGL
// attribute pointers
impl Attribute {
    pub fn new<S>(program_id: GLuint, name: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let mut location = 0;
        let mut buffer_id = 0;
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
            buffer_id = vbo;

            // Convert to GLuint from GLint
            location = attribute_location
                .try_into()
                .map_err(|_| VAOError::FailedIDConversion)?;
        }
        Ok(Attribute {
            location,
            buffer_id,
        })
    }
}
// Arrays of Vec3<f32>'s 'know' how to set up their attribute pointers
pub struct GLAV3(pub Vec<GL3F>);
pub trait SetAttributePointer {
    // TODO: This doesn't need to return anything anymore
    fn set_attribute_pointer(&self, id: GLuint) -> Result<()>;
}
impl SetAttributePointer for GL3FV {
    fn set_attribute_pointer(&self, id: GLuint) -> Result<()> {
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const std::ffi::c_void;
        let size = (self.0.len() * std::mem::size_of::<GL3F>()) as GLsizeiptr;
        let element_size = (std::mem::size_of::<GL3F>()) as GLsizei;
        unsafe {
            // Send the data to the GPU
            gl::BufferData(gl::ARRAY_BUFFER, size, ptr, gl::STATIC_DRAW);
            // Tell OpenGL how to pull data from the buffer into the attributes inside the shaders
            gl::VertexAttribPointer(id, 3, gl::FLOAT, gl::FALSE, element_size, ptr::null());
        }
        Ok(())
    }
}

impl SetAttributePointer for Vec<Vec2> {
    fn set_attribute_pointer(&self, id: GLuint) -> Result<()> {
        // Pointer to the vector's buffer
        let ptr = self.as_ptr() as *const std::ffi::c_void;
        let size = (self.len() * std::mem::size_of::<Vec2>()) as GLsizeiptr;
        let element_size = (std::mem::size_of::<Vec2>()) as GLsizei;
        unsafe {
            // Send the data to the GPU
            gl::BufferData(gl::ARRAY_BUFFER, size, ptr, gl::STATIC_DRAW);
            // Tell OpenGL how to pull data from the buffer into the attributes inside the shaders
            gl::VertexAttribPointer(id, 2, gl::FLOAT, gl::FALSE, element_size, ptr::null());
        }
        Ok(())
    }
}

impl SetAttributePointer for Vec<Vec3> {
    fn set_attribute_pointer(&self, id: GLuint) -> Result<()> {
        // Pointer to the vector's buffer
        let ptr = self.as_ptr() as *const std::ffi::c_void;
        let size = (self.len() * std::mem::size_of::<Vec3>()) as GLsizeiptr;
        let element_size = (std::mem::size_of::<Vec3>()) as GLsizei;
        unsafe {
            // Send the data to the GPU
            gl::BufferData(gl::ARRAY_BUFFER, size, ptr, gl::STATIC_DRAW);
            // Tell OpenGL how to pull data from the buffer into the attributes inside the shaders
            gl::VertexAttribPointer(id, 3, gl::FLOAT, gl::FALSE, element_size, ptr::null());
        }
        Ok(())
    }
}

impl SetAttributePointer for Vec<Vec4> {
    fn set_attribute_pointer(&self, id: GLuint) -> Result<()> {
        // Pointer to the vector's buffer
        let ptr = self.as_ptr() as *const std::ffi::c_void;
        let size = (self.len() * std::mem::size_of::<Vec4>()) as GLsizeiptr;
        let element_size = (std::mem::size_of::<Vec4>()) as GLsizei;
        unsafe {
            // Send the data to the GPU
            gl::BufferData(gl::ARRAY_BUFFER, size, ptr, gl::STATIC_DRAW);
            // Tell OpenGL how to pull data from the buffer into the attributes inside the shaders
            gl::VertexAttribPointer(id, 4, gl::FLOAT, gl::FALSE, element_size, ptr::null());
        }
        Ok(())
    }
}
