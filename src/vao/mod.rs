// Custom Error Type
pub mod error;
pub use crate::program::Attribute;
pub use error::VAOError;
// Types we defing "SetAttributePointer"
// TODO: Implement for all types, including ultraviolet types
use crate::uniform::GL3F;
use crate::uniform::GL3FV;

// OpenGL Types
use gl::types::*;

// Used to map Attributes to their identifiers in shader GLSL code
use std::collections::hash_map::Entry;
use std::{collections::HashMap, vec::Vec};
// Used for casting into the OpenGL library
use std::ptr;

use ultraviolet::vec::{Vec2, Vec3, Vec4};

// Arrays of Vec3<f32>'s 'know' how to set up their attribute pointers
pub struct GLAV3(pub Vec<GL3F>);
pub trait SetAttributePointer {
    fn set_attribute_pointer(&mut self, id: GLuint) -> Result<GLint, VAOError>;
}
impl SetAttributePointer for GL3FV {
    fn set_attribute_pointer(&mut self, id: GLuint) -> Result<GLint, VAOError> {
        self.0.shrink_to_fit();
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
        Ok(self.0.len() as i32)
    }
}

impl SetAttributePointer for Vec<Vec2> {
    fn set_attribute_pointer(&mut self, id: GLuint) -> Result<GLint, VAOError> {
        self.shrink_to_fit();
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
        Ok(self.len() as i32)
    }
}

impl SetAttributePointer for Vec<Vec3> {
    fn set_attribute_pointer(&mut self, id: GLuint) -> Result<GLint, VAOError> {
        self.shrink_to_fit();
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
        Ok(self.len() as i32)
    }
}

impl SetAttributePointer for Vec<Vec4> {
    fn set_attribute_pointer(&mut self, id: GLuint) -> Result<GLint, VAOError> {
        self.shrink_to_fit();
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
        Ok(self.len() as i32)
    }
}

// Represents an OpenGL Vertex Array Object - provides a handle to the VAO
// and allows attaching attributes to it
#[derive(Debug)]
pub struct VAO {
    // GL ID of this VAO, and the name of this VAO
    pub id: GLuint,
    pub program_id: GLuint,
    pub enabled: bool,
    pub ele_buffer: GLuint,
    // List of named attributes and their GL IDs
    pub attributes: HashMap<String, Attribute>,
    // How to draw buffers
    pub draw_style: GLuint,
    pub draw_count: GLint,
}

impl VAO {
    // Create a new VAO, using an ID created by a GLProgram
    pub fn new(program_id: GLuint, indices: Vec<u32>) -> Self {
        // Enabled by default
        let mut id = 0;
        let enabled = true;
        let attributes = HashMap::new();
        let draw_style = gl::TRIANGLES;
        let draw_count = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        // Set up the VAO state to use indices
        let mut ele_buffer = 0;
        let ele_buffer_ptr = indices.as_ptr() as *const std::ffi::c_void;
        let ele_buffer_size = (indices.len() * std::mem::size_of::<u32>()) as isize;
        unsafe {
            gl::GenBuffers(1, &mut ele_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ele_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ele_buffer_size,
                ele_buffer_ptr,
                gl::STATIC_DRAW,
            );
        }

        VAO {
            id,
            program_id,
            enabled,
            ele_buffer,
            attributes,
            draw_style,
            draw_count,
        }
    }

    // Attaches a buffer to a named attribute location in the shader code, and informs
    // OpenGL how to pull from it.
    pub fn attribute<S, B>(&mut self, name: S, mut buffer: B) -> Result<&mut VAO, VAOError>
    where
        S: AsRef<str>,
        B: SetAttributePointer,
    {
        // let program_id = get_program_id()?;
        let attribute = match self.attributes.entry(name.as_ref().to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Attribute::new(self.program_id, name)?),
        };

        unsafe {
            gl::UseProgram(self.program_id);
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, attribute.buffer);
            gl::EnableVertexAttribArray(attribute.id);

            // Sets up how to pull from the buffer, and how many times to pull from the buffer
            self.draw_count = buffer.set_attribute_pointer(attribute.id)?;

            // Unbind Targets
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(self)
    }
}
