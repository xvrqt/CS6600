// #![allow(dead_code)]
// Import and Re-Export our Error Type
pub mod error;
pub use error::ProgramError;
pub mod blinn_phong;
pub mod builder;

pub mod lights;
pub use lights::{LightColor, LightSource, Position};

pub use camera::Projection;
pub use window::FrameState;

use crate::window::GLWindow;
type Result<T> = std::result::Result<T, ProgramError>;

pub mod camera;
use crate::window;

// Programs must attach at least a Vertex and Fragment Shader
use crate::shader::ShaderPipeline;
// Create and set uniform shader values
use crate::uniform::Uniform;
use blinn_phong::BlinnPhong;

// OpenGL Types
use gl::types::*;

// Used by OpenGL functions to look up locations of uniforms and attributes in shaders
use std::ffi::CString;

use self::camera::Camera;
use fragment_only::FragmentOnly;
mod fragment_only;

// Semantic OpenGL Program
// #[derive(Debug)]
#[allow(dead_code)]

// V, F -> Shader Type (built-in shaders have special implementations to make things easier)
pub struct GLProgram<'a, Type> {
    // OpenGL Program ID
    id: u32,
    // Window, Events, and OpenGL context
    context: GLWindow,
    // OpenGL Shaders, e.g. vertex, fragment, et al.
    shaders: ShaderPipeline<'a>,
    // Different types data based on the Shader type
    data: Type,
}

// All GLProgram Types have to implement a standard draw() call which draws the program contents to
// its context/window.
pub trait GLDraw {
    fn draw(&mut self) -> Result<()>;
}

// Dummy types that represent different GLPrograms with different abilities built-in
#[derive(Debug)]
pub struct CustomShader;

// Functions commong to all GLProgram types
impl<'a, Any> GLProgram<'a, Any> {
    // Sets a uniform variable at the location
    pub fn set_uniform<S, Type>(&self, name: S, mut value: Type) -> Result<()>
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

    // Convenience function to look up uniform locatoin
    fn get_uniform_location<S>(&self, name: S) -> Result<GLint>
    where
        S: AsRef<str>,
    {
        let c_name = CString::new(name.as_ref()).map_err(|_| {
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

    // Similar to get_uniform_location but for block indices
    fn get_uniform_block_index<S>(&self, name: S) -> Result<GLuint>
    where
        S: AsRef<str>,
    {
        let c_name = CString::new(name.as_ref()).map_err(|_| {
            ProgramError::SettingUniformValue(
                "Could not create CString from the uniform's location name.".to_string(),
            )
        })?;
        let location;
        unsafe {
            location = gl::GetUniformBlockIndex(self.id, c_name.into_raw());
        }
        match location {
            gl::INVALID_INDEX => Err(ProgramError::GetUniformLocation(name.as_ref().into())),
            _ => Ok(location),
        }
    }
}
