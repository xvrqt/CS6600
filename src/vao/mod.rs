pub mod error;
pub use error::VAOError;
use gl::types::*;
use std::collections::hash_map::Entry;
use std::{collections::HashMap, vec::Vec};

pub struct Vertex3(pub Vec<f32>);
pub trait SetAttributePointer {
    fn set_attribute_pointer(&mut self) -> Result<(), VAOError>;
}
impl SetAttributePointer for Vertex3 {
    fn set_attribute_pointer(&mut self) -> Result<(), VAOError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| VAOError::VectorLength)?;

        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const std::ffi::c_void;
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.0.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &self.0[0] as *const f32 as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<GLfloat>() as GLsizei,
                std::ptr::null(),
            );
        }
        Ok(())
    }
}

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
            // Get the ID of the currently running program
            // let program_id = get_program_id()?;

            // Get the handle to where the attribute is located in the shader
            let name = std::ffi::CString::new(name.as_ref().as_bytes())
                .map_err(|_| VAOError::CStringConversion)?;
            let attribute_location = gl::GetAttribLocation(program_id, name.as_ptr());
            if attribute_location < 0 {
                return Err(VAOError::CouldNotFindLocation(
                    name.to_string_lossy().into_owned(),
                ));
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

    pub fn set_attribute_pointer(&self) -> Result<(), VAOError> {
        Ok(())
    }
}

fn get_program_id() -> Result<GLuint, VAOError> {
    unsafe {
        // Get the ID of the currently running program
        let mut program_id = 1;
        gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut program_id);
        program_id
            .try_into()
            .map_err(|_| VAOError::FailedToGetActiveProgram)
    }
}

#[derive(Debug)]
pub struct VAO {
    // GL ID of this VAO, and the name of this VAO
    pub id: GLuint,
    pub program_id: GLuint,
    pub enabled: bool,
    // List of named attributes and their GL IDs
    attributes: HashMap<String, Attribute>,
}

impl VAO {
    // Create a new VAO, using an ID created by a GLProgram
    pub fn new(program_id: GLuint) -> Self {
        // Enabled by default
        let mut id = 0;
        let enabled = true;
        let attributes = HashMap::new();

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VAO {
            id,
            program_id,
            enabled,
            attributes,
        }
    }

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
            buffer.set_attribute_pointer()?;
            gl::EnableVertexAttribArray(self.id);
            // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::BindVertexArray(0);
        }

        Ok(self)
    }
}
