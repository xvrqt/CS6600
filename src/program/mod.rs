// #![allow(dead_code)]
// Import and Re-Export our Error Type
pub mod blinn_phong;
pub mod builder;
pub mod camera;
pub mod error;
mod fragment_only;
pub mod lights;
pub mod mesh;
pub mod scene_object;
pub mod vao;

use crate::interface_blocks::UniformBufferBlock;
pub use crate::interface_blocks::{InterfaceBlock, InterfaceBuffer};
use crate::shader::ShaderPipeline;
pub use crate::uniform::UpdateUniform;
use crate::uniform::{Uniform, UniformValue};
use crate::window;
use blinn_phong::BlinnPhong;
pub use camera::{Camera, Projection};
pub use error::ProgramError;
use fragment_only::FragmentOnly;
pub use lights::{LightColor, LightSource, Position};
pub use mesh::Mesh;
pub use vao::attribute::Attribute;
pub use window::{FrameState, GLWindow};

// Error Types
type Result<T> = std::result::Result<T, ProgramError>;

// OpenGL Types
use gl::types::*;

// Used by OpenGL functions to look up locations of uniforms and attributes in shaders
use std::collections::HashMap;
use std::ffi::CString;
use std::rc::{Rc, Weak};

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
    // Uniform locations, and their values
    uniforms: HashMap<Rc<str>, Rc<dyn UpdateUniform>>,
    interface_blocks: HashMap<Rc<str>, Rc<dyn InterfaceBuffer>>,
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
    //////////////
    // UNIFORMS //
    //////////////
    pub fn attach_uniform<'b, Value>(
        &mut self,
        uniform: Uniform<Value>,
    ) -> Result<Weak<dyn UpdateUniform>>
    where
        Value: UniformValue + 'static,
    {
        unsafe {
            gl::UseProgram(self.id);
        }

        let key = uniform.key();
        let value = uniform.attach(self.id)?;
        let weak = Rc::downgrade(&value);
        self.uniforms.insert(key, value);
        Ok(weak)
    }

    pub fn attach_interface_block<'b, Value>(
        &mut self,
        block: InterfaceBlock<
            crate::interface_blocks::Uniform,
            crate::interface_blocks::Std140,
            Value,
            crate::interface_blocks::Unattached<Value>,
        >,
    ) -> Result<
        InterfaceBlock<
            crate::interface_blocks::Uniform,
            crate::interface_blocks::Std140,
            Value,
            crate::interface_blocks::Attached,
        >,
    > {
        unsafe {
            gl::UseProgram(self.id);
        }

        let key = block.key();
        // hardcoded :[
        let value = block.attach(self.id, 1)?;
        Ok(value)
    }

    // Creates a new uniform, initializes it in the GLProgram and adds it to the HashMap
    pub fn create_uniform<'b, S, Value>(
        &mut self,
        name: S,
        value: &'b Value,
    ) -> Result<Weak<dyn UpdateUniform>>
    where
        S: AsRef<str>,
        Value: UniformValue + 'static,
    {
        let uniform = Uniform::new(name, value)?;
        self.attach_uniform(uniform)
    }

    // Creates a new Interface Block, initializes it in the GLProgram and adds it to the HashMap
    pub fn create_interface_block<S, Value>(
        &mut self,
        value: Vec<Value>,
    ) -> Result<
        InterfaceBlock<
            crate::interface_blocks::Uniform,
            crate::interface_blocks::Std140,
            Value,
            crate::interface_blocks::Attached,
        >,
    >
    where
        S: AsRef<str>,
        Value: UniformValue + 'static,
    {
        let interface_block = UniformBufferBlock::new_std140("lights", value)?;
        self.attach_interface_block(interface_block)
    }

    // Returns a weak reference to an existing Uniform in the program (which the caller can then
    // call .update() on to update the value in the program).
    pub fn get_uniform<S>(&self, name: &S) -> Option<Weak<dyn UpdateUniform>>
    where
        S: AsRef<str>,
    {
        self.uniforms
            .get(name.as_ref())
            .map(|uniform| Rc::downgrade(uniform))
    }

    // Lazy way to combine getting and updating and existing uniform
    pub fn update_uniform<S, Value>(&self, name: S, value: &Value) -> Result<()>
    where
        S: AsRef<str>,
        Value: UniformValue,
    {
        unsafe {
            gl::UseProgram(self.id);
        }

        self.get_uniform(&name)
            .and_then(|uniform| uniform.upgrade())
            .and_then(|uniform| Some(uniform.update(value)))
            .ok_or(ProgramError::UniformNotAttachedToProgram(
                name.as_ref().to_string(),
            ))
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
