#![allow(dead_code)]
// Import the type we're building (GLProgram) and our Error Type
use super::error::ProgramError;
use super::GLProgram;
use crate::shader::{CustomFragmentShader, CustomVertexShader};

// Programs must attach at least a Vertex and Fragment Shader
use crate::shader::{Fragment, Shader, Vertex};

// OpenGL Types
use gl::types::*;
// Used to track Vertex Array Objects use std::collections::HashMap;
// Used by OpenGL functions to look up locations of uniforms and attributes in shaders
// Used to create error logs
use std::vec::Vec;

// Using a typesetting builder pattern to create valid OpenGL Programs
// V: Vertex Shader Type
// F: Fragment Shader Type
pub struct GLProgramBuilder<V, F> {
    pub(crate) id: GLuint,
    pub(crate) vertex_shader: V,
    pub(crate) fragment_shader: F,
}

// Dummy types; Since a Vertex and a Fragment shader are mandatory,
// use typesetting to ensure the user builds a valie GLProgram at compile time
// TODO: Can I hide these from users ?

// If the caller hasn't attached a vertex or fragment shader
pub struct NoVS;
pub struct NoFS;

// Built-in shader variants
pub use crate::shader::blinn_phong::{BlinnPhongFragmentShader, BlinnPhongVertexShader};

// If we haven't attached any shaders, allow the user to select a builtin shader
impl<'a> GLProgramBuilder<NoVS, NoFS> {
    pub fn blinn_phong_shading(
        self,
    ) -> Result<GLProgram<BlinnPhongVertexShader<'a>, BlinnPhongFragmentShader<'a>>, ProgramError>
    {
        // Compile Blinn-Phong Shaders
        let vertex_shader = Shader::<Vertex>::blinn_phong()?;
        let fragment_shader = Shader::<Fragment>::blinn_phong()?;

        // Attach them to this program, and link them
        unsafe {
            gl::AttachShader(self.id, vertex_shader.id);
            gl::AttachShader(self.id, fragment_shader.id);

            gl::LinkProgram(self.id);
        }

        // If it was a success, return the Blinn-Phong builder
        link_shaders_success(self.id).and_then(|_| Ok(self.into()))
    }
}

// If both a vertex shader and a fragment shader are present link them and return a Custom GLProgram
impl<'a> GLProgramBuilder<CustomVertexShader<'a>, CustomFragmentShader<'a>> {
    // Links vertex and fragment shader to an OpenGL program
    pub fn link_shaders(
        self,
    ) -> Result<GLProgram<CustomVertexShader<'a>, CustomFragmentShader<'a>>, ProgramError> {
        unsafe {
            gl::LinkProgram(self.id);
        }

        // If it was a success, return a GLProgram from the builder
        link_shaders_success(self.id).and_then(|_| Ok(self.into()))
    }
}

// If we haven't attached a Vertex Shader provide a method to attach one
impl<F> GLProgramBuilder<NoVS, F> {
    pub fn attach_vertex_shader(
        self,
        vertex_shader: CustomVertexShader,
    ) -> GLProgramBuilder<CustomVertexShader, F> {
        unsafe { gl::AttachShader(self.id, vertex_shader.id) }
        let Self {
            id,
            fragment_shader,
            ..
        } = self;
        GLProgramBuilder {
            id,
            vertex_shader,
            fragment_shader,
        }
    }
}

// If we haven't attached a Fragment Shader provide a method to attach one
impl<V> GLProgramBuilder<V, NoFS> {
    pub fn attach_fragment_shader(
        self,
        fragment_shader: CustomFragmentShader,
    ) -> GLProgramBuilder<V, CustomFragmentShader> {
        unsafe { gl::AttachShader(self.id, fragment_shader.id) }
        let Self {
            id, vertex_shader, ..
        } = self;
        GLProgramBuilder {
            id,
            vertex_shader,
            fragment_shader,
        }
    }
}

// Helper function that checks if linking the shaders to the program was a success
pub(crate) fn link_shaders_success(program_id: GLuint) -> Result<(), ProgramError> {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            // Determine the log's length
            let mut length = 0 as GLint;
            gl::GetShaderiv(program_id, gl::INFO_LOG_LENGTH, &mut length);
            let log_length: usize = length.try_into().map_err(|_| {
                ProgramError::Linking(String::from("Couldn't determine length of error log."))
            })?;

            // Set up a buffer to receive the log
            let mut error_log = Vec::<u8>::with_capacity(log_length);
            if log_length > 0 {
                error_log.set_len(log_length - 1);
            } // Don't read the NULL terminator

            gl::GetProgramInfoLog(
                program_id,
                512,
                std::ptr::null_mut(),
                error_log.as_mut_ptr() as *mut GLchar,
            );

            // Return the error log and exit
            Err(ProgramError::Linking(
                std::str::from_utf8(&error_log).unwrap().into(),
            ))
        } else {
            Ok(())
        }
    }
}
