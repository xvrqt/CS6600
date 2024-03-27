#![allow(dead_code)]
// Import the type we're building (GLProgram) and our Error Type
use super::error::ProgramError;
use super::GLProgram;

// Programs must attach at least a Vertex and Fragment Shader
use crate::shader::{Fragment, Shader, Vertex};

// Create and set uniform shader values
use crate::uniform::MagicUniform;

// OpenGL Types
use gl::types::*;
// Used to track Vertex Array Objects use std::collections::HashMap;
// Used by OpenGL functions to look up locations of uniforms and attributes in shaders
// Used to create error logs
use std::collections::HashMap;
use std::vec::Vec;

// Using a typesetting builder pattern to create valid OpenGL Programs
// T: Type of Shaders (Builtin or Custom)
// V: Vertex Shader Type
// F: Fragment Shader Type
pub struct GLProgramBuilder<T, V, F> {
    pub(crate) id: GLuint,
    pub(crate) vertex_shader: V,
    pub(crate) fragment_shader: F,
    _pd: std::marker::PhantomData<T>,
    // pub(crate) camera: C,
    // pub(crate) lights: L,
}

// Dummy types; Since a Vertex and a Fragment shader are mandatory,
// use typesetting to ensure the user builds a valie GLProgram at compile time
// TODO: Can I hide these from users ?
// T
pub struct CustomShader;
pub struct BuiltInShader;

// T -> BuiltinShader variants
pub use crate::shader::blinn_phong::{BlinnPhongFragmentShader, BlinnPhongVertexShader};

// T -> CustomShader
pub struct NoVS;
pub struct NoFS;
//
// pub struct NoLight;
// pub struct NoCamera;

// Helper function that checks if linking the shaders to the program was a success
fn link_shaders_success(program_id: GLuint) -> Result<(), ProgramError> {
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

// If we haven't attached any shaders, allow the user to select a builtin shader
impl<'a> GLProgramBuilder<CustomShader, NoVS, NoFS> {
    pub fn blinn_phong_shading(
        self,
    ) -> Result<
        GLProgramBuilder<BuiltInShader, BlinnPhongVertexShader<'a>, BlinnPhongFragmentShader<'a>>,
        ProgramError,
    > {
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
        link_shaders_success(self.id).and_then(|_| {
            Ok(GLProgramBuilder {
                id: self.id,
                vertex_shader,
                fragment_shader,
                _pd: std::marker::PhantomData,
            })
        })
    }
}

// If both a vertex shader and a fragment shader are present link them and return a Custom GLProgram
impl<'a> GLProgramBuilder<CustomShader, Shader<'a, Vertex>, Shader<'a, Fragment>> {
    // Links vertex and fragment shader to an OpenGL program
    pub fn link_shaders(self) -> Result<GLProgram<'a>, ProgramError> {
        unsafe {
            gl::LinkProgram(self.id);
        }

        // If it was a success, return a GLProgram
        link_shaders_success(self.id).and_then(|_| {
            let Self {
                id,
                fragment_shader,
                vertex_shader,
                ..
            } = self;

            Ok(GLProgram {
                id: self.id,
                vertex_shader,
                fragment_shader,
                magic_uniforms: MagicUniform::NONE,
                vaos: HashMap::new(),
            })
        })
    }
}

// If we haven't attached a Vertex Shader provide a method to attach one
impl<F> GLProgramBuilder<CustomShader, NoVS, F> {
    pub fn attach_vertex_shader(
        self,
        vs: Shader<Vertex>,
    ) -> GLProgramBuilder<CustomShader, Shader<Vertex>, F> {
        unsafe { gl::AttachShader(self.id, vs.id) }
        let Self {
            id,
            fragment_shader,
            ..
        } = self;
        GLProgramBuilder {
            id,
            vertex_shader: vs,
            fragment_shader,
            _pd: std::marker::PhantomData,
        }
    }
}

// If we haven't attached a Fragment Shader provide a method to attach one
impl<V> GLProgramBuilder<CustomShader, V, NoFS> {
    pub fn attach_fragment_shader(
        self,
        fs: Shader<Fragment>,
    ) -> GLProgramBuilder<CustomShader, V, Shader<Fragment>> {
        unsafe { gl::AttachShader(self.id, fs.id) }
        let Self {
            id, vertex_shader, ..
        } = self;
        GLProgramBuilder {
            id,
            vertex_shader,
            fragment_shader: fs,
            _pd: std::marker::PhantomData,
        }
    }
}
