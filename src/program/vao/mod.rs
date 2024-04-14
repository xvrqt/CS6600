// Custom Error Type
use std::fmt::Debug;
pub mod error;
pub use crate::program::Attribute;
pub use error::VAOError;
// Types we defing "SetAttributePointer"
// TODO: Implement for all types, including ultraviolet types
pub mod attribute;
use attribute::SetAttributePointer;
type Result<T> = std::result::Result<T, VAOError>;

// OpenGL Types
use gl::types::*;

// Used to map Attributes to their identifiers in shader GLSL code
use std::collections::hash_map::Entry;
use std::{collections::HashMap, vec::Vec};
// Used for casting into the OpenGL library
use std::ffi::c_void;
use std::mem::size_of;

#[derive(Debug, Clone)]
pub struct ElementIndices {
    // The buffer that holds the element indices used for OpenGL's `glDrawElements()` function
    pub buffer_id: GLuint,
    // The number of elements to render (essentially `ele_buffer.len()`)
    pub buffer_length: GLint,
}

// Represents an OpenGL Vertex Array Object - provides a handle to the VAO
// and allows attaching attributes to it
#[derive(Debug, Clone)]
pub struct VAO {
    // GL ID of this VAO, and the name of this VAO
    pub id: GLuint,
    // VAO's can't exist without a GLProgram for context. We require this in the creator to ensure
    // we a GLProgram exists, and a convenience for adding Attributes to the VAO (i.e. we don't
    // ahve to pass the program ID in to every call)
    pub program_id: GLuint,
    pub elements: ElementIndices,
    // List of named attributes and their OpenGL locations, and buffer IDs
    pub attributes: HashMap<String, Attribute>,
}

impl VAO {
    // Create a new VAO for for a GLProgram. Pass the elements array to be sent to the GPU, and a
    // list of attributes to be be associated with this VAO, and connected to the element array
    pub fn new(program_id: GLuint, indices: &Vec<u32>) -> Result<Self> {
        // Create our new VAO
        let id = Self::get_id();
        let elements = Self::create_element_array(indices);
        let vao = VAO {
            id,
            elements,
            program_id,
            attributes: HashMap::new(),
        };

        Ok(vao)
    }

    // Convenience
    #[inline(always)]
    fn get_id() -> GLuint {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        id
    }

    fn create_element_array(buffer: &Vec<u32>) -> ElementIndices {
        let ele_buffer_ptr = buffer.as_ptr() as *const c_void;
        let ele_buffer_size = (buffer.len() * size_of::<u32>()) as isize;
        let mut buffer_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ele_buffer_size,
                ele_buffer_ptr,
                gl::STATIC_DRAW,
            );
        }
        let buffer_length = buffer.len() as GLint;
        ElementIndices {
            buffer_id,
            buffer_length,
        }
    }

    // Attaches a buffer to a named attribute location in the shader code, and informs
    // OpenGL how to pull from it.
    pub fn add_attribute<S, B>(&mut self, name: S, buffer: &B, instanced: bool) -> Result<&mut VAO>
    where
        S: AsRef<str>,
        B: SetAttributePointer + std::fmt::Debug,
    {
        // let program_id = get_program_id()?;
        let attribute = match self.attributes.entry(name.as_ref().to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Attribute::new(self.program_id, name)?),
        };

        unsafe {
            gl::UseProgram(self.program_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, attribute.buffer_id);
            gl::BindVertexArray(self.id);
            gl::EnableVertexAttribArray(attribute.location);

            // Sets up how to pull from the buffer, and how many times to pull from the buffer
            buffer.set_attribute_pointer(attribute.location)?;

            // Unbind Targets
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(self)
    }

    // Attaches a buffer to a named attribute location in the shader code, and informs
    // OpenGL how to pull from it.
    pub fn update_attribute<S, B>(&self, name: S, buffer: &B, instanced: bool) -> Result<()>
    where
        S: AsRef<str>,
        B: SetAttributePointer + std::fmt::Debug,
    {
        let attribute = self
            .attributes
            .get(name.as_ref())
            .ok_or(VAOError::CouldNotFindAttribute(name.as_ref().to_string()))?;

        unsafe {
            gl::UseProgram(self.program_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, attribute.buffer_id);
            gl::BindVertexArray(self.id);
            gl::EnableVertexAttribArray(attribute.location);

            // Sets up how to pull from the buffer, and how many times to pull from the buffer
            buffer.set_attribute_pointer(attribute.location)?;

            // Unbind Targets
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Ok(())
    }
}
