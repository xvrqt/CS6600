// Import and Re-Export our Error Type
pub mod error;
pub use error::ProgramError;

use crate::types::*;
use crate::uniform::{MagicUniform, Uniform};
use crate::window::FrameEvents;

// We need the Shader type to link them to our id
use crate::shader::{Fragment, Shader, Vertex};

// OpenGL
use gl::types::*;

use std::ffi::CString;

// Semantic OpenGL Program
#[derive(Debug)]
#[allow(dead_code)]
pub struct GLProgram<'a> {
    id: u32, // OpenGL keeps track of programs with integer IDs
    vertex_shader: Shader<'a, Vertex>,
    fragment_shader: Shader<'a, Fragment>,
    // List of uniforms we update automagically for the caller
    magic_uniforms: MagicUniform,
}

impl GLProgram<'_> {
    pub fn builder() -> GLProgramBuilder<NoVS, NoFS> {
        let id;
        unsafe {
            id = gl::CreateProgram();
        }
        GLProgramBuilder {
            id,
            vertex_shader: NoVS,
            fragment_shader: NoFS,
        }
    }

    // Enables a magic uniform value
    pub fn enable_uniform(mut self, uniform: MagicUniform) -> Self {
        self.magic_uniforms = self.magic_uniforms | uniform;
        self
    }

    // Checks which magic uniforms are enabled and then sets them accordingly
    fn update_magic_uniforms(&self, vars: &FrameEvents) -> Result<(), ProgramError> {
        if self.magic_uniforms.contains(MagicUniform::TIME) {
            self.set_uniform("time", GL1F(vars.time))?;
        }
        if self.magic_uniforms.contains(MagicUniform::RESOLUTION) {
            self.set_uniform("resolution", GL2F(vars.resolution.0, vars.resolution.1))?;
        }
        Ok(())
    }

    // Updates the magic uniforms, draws every VAO in order
    pub fn draw(&self, frame_events: &FrameEvents) -> Result<(), ProgramError> {
        self.update_magic_uniforms(&frame_events)?;
        Ok(())
    }

    // Sets a uniform variable at the location
    pub fn set_uniform<S, Type>(&self, name: S, mut value: Type) -> Result<(), ProgramError>
    where
        S: AsRef<str>,
        Type: Uniform,
    {
        unsafe {
            gl::UseProgram(self.id);
        }
        let location = self.get_uniform_location(name)?;
        value
            .set(location)
            .map_err(|e| ProgramError::SettingUniformValue(e.to_string()))?;

        Ok(())
    }

    // Convenience
    fn get_uniform_location<S>(&self, name: S) -> Result<GLint, ProgramError>
    where
        S: AsRef<str>,
    {
        let c_name = CString::new(name.as_ref().as_bytes()).map_err(|_| {
            ProgramError::SettingUniformValue(
                "Could not create CString from the uniform's location name.".to_string(),
            )
        })?;
        let location;
        unsafe {
            location = gl::GetUniformLocation(self.id, c_name.into_raw());
        }
        match location {
            -1 => Err(ProgramError::GetUniformLocation(name.as_ref().into())),
            _ => Ok(location),
        }
    }

    // Returns the OpenGL ID of the id
    pub fn id(&self) -> GLuint {
        self.id
    }
}

// Using a typesetting builder pattern to create valid OpenGL Programs
pub struct GLProgramBuilder<V, F> {
    id: GLuint,
    vertex_shader: V,
    fragment_shader: F,
}

// Dummy types; Since a Vertex and a Fragment shader are mandatory,
// use typesetting to ensure the user builds a valie GLProgram at compile time
pub struct NoVS;
pub struct NoFS;

// If we haven't attached a Vertex Shader provide a method to attach one
impl<F> GLProgramBuilder<NoVS, F> {
    pub fn attach_vertex_shader(self, vs: Shader<Vertex>) -> GLProgramBuilder<Shader<Vertex>, F> {
        unsafe { gl::AttachShader(self.id, vs.id()) }
        let Self {
            id,
            fragment_shader,
            ..
        } = self;
        GLProgramBuilder {
            id,
            vertex_shader: vs,
            fragment_shader,
        }
    }
}

// If we haven't attached a Fragment Shader provide a method to attach one
impl<V> GLProgramBuilder<V, NoFS> {
    pub fn attach_fragment_shader(
        self,
        fs: Shader<Fragment>,
    ) -> GLProgramBuilder<V, Shader<Fragment>> {
        unsafe { gl::AttachShader(self.id, fs.id()) }
        let Self {
            id, vertex_shader, ..
        } = self;
        GLProgramBuilder {
            id,
            vertex_shader,
            fragment_shader: fs,
        }
    }
}

// If both a vertex shader and a fragment shader are present, allow linking
impl<'a> GLProgramBuilder<Shader<'a, Vertex>, Shader<'a, Fragment>> {
    pub fn link_shaders(self) -> Result<GLProgram<'a>, ProgramError> {
        unsafe {
            gl::LinkProgram(self.id);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                // Determine the log's length
                let mut length = 0 as GLint;
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut length);
                let log_length: usize = length.try_into().map_err(|_| {
                    ProgramError::Linking(String::from("Couldn't determine length of error log."))
                })?;

                // Set up a buffer to receive the log
                let mut error_log = Vec::<u8>::with_capacity(log_length);
                if log_length > 0 {
                    error_log.set_len(log_length - 1);
                } // Don't read the NULL terminator
                gl::GetProgramInfoLog(
                    self.id,
                    512,
                    std::ptr::null_mut(),
                    error_log.as_mut_ptr() as *mut GLchar,
                );

                // Return the error log and exit
                return Err(ProgramError::Linking(
                    std::str::from_utf8(&error_log).unwrap().into(),
                ));
            }
        }
        // Copy our previous values into a new struct, and return
        let Self {
            id,
            fragment_shader,
            vertex_shader,
            ..
        } = self;

        Ok(GLProgram {
            id,
            vertex_shader,
            fragment_shader,
            magic_uniforms: MagicUniform::NONE,
        })
    }
}
